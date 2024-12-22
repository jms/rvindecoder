extern crate dotenv;

use convert_case::{Case, Casing};
use dotenv::dotenv;
use std::env;

use axum::{
    body::Bytes,
    extract::{MatchedPath, State},
    http::{HeaderMap, Request, StatusCode},
    response::{Html, Response},
    routing::get,
    Json, Router,
};

use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use bb8::Pool;
use bb8_tiberius::ConnectionManager;

use tiberius::{AuthMethod, Config};

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct DecodedData {
    pub variable: String,
    pub value: String,
}

fn get_connection_manager() -> ConnectionManager {
    dotenv().ok();
    let mssql_host = env::var("DB_HOST").expect("host must be set");
    let mssql_user = env::var("DB_USERNAME").expect("username must be set");
    let mssql_password = env::var("DB_PASSWORD").expect("password must be set");
    let db_name = env::var("DB_NAME").expect("db name must be set");

    let mut config = Config::new();
    config.host(mssql_host);
    config.port(1433);
    config.authentication(AuthMethod::sql_server(mssql_user, mssql_password));
    config.database(db_name);
    config.trust_cert();

    let manager = ConnectionManager::build(config).unwrap();
    manager
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "rvindecoder=debug,tower_http=debug,axum::rejection=trace",
        )
    }
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "rvindecoder=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = get_connection_manager();
    let pool = bb8::Pool::builder()
        .max_size(4)
        .build(manager)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/decode/:vin", get(decode_vin))
        .with_state(pool)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

type ConnectionPool = Pool<ConnectionManager>;

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    tracing::info!("hit root");
    Html("<h1>Rust VIN Decoder APIr</h1>")
}

async fn decode_vin(
    State(pool): State<ConnectionPool>,
    axum::extract::Path(vin): axum::extract::Path<String>,
) -> Result<Json<HashMap<String, String>>, (StatusCode, String)> {
    tracing::info!("VIN submitted: {:?}", vin);
    let mut conn = pool.get().await.map_err(internal_error)?;
    // 3FA6P0LU1KR165357
    let rows = conn
        .query("exec dbo.spVinDecode @v = @P1", &[&vin])
        .await
        .map_err(internal_error)?
        .into_first_result()
        .await
        .map_err(internal_error)?;

    let mut decoded_data = Vec::<DecodedData>::new();

    for row in rows.into_iter() {
        let key: &str = row.get("Variable").unwrap();
        let value: &str = row.get("Value").unwrap_or("");
        decoded_data.push(DecodedData {
            variable: key.to_string().to_case(Case::Snake),
            value: value.to_string(),
        });
    }
    let data: HashMap<_, _> = decoded_data
        .iter()
        .map(|x| (x.variable.clone(), x.value.clone()))
        .collect();
    // tracing::debug!("{:?}", data);
    Ok(Json(data))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
