### Rust VIN Decoder

rvindecoder is a hobby project, a web application in Rust to decode VIN numbers inspired 
by NHTSA and the data available, for reference

https://vpic.nhtsa.dot.gov/

https://vpic.nhtsa.dot.gov/api/

The application was made to try out axum, bb8 and tiberius packages. 

the repository includes a script to load the data `load-data.sh` in a docker container
using the image `mcr.microsoft.com/azure-sql-edge` and get the DB up

To run the app:

add an `.env` file with the following:
```dotenv
DB_HOST=localhost
DB_PORT=1433
DB_USERNAME="sa"
DB_PASSWORD="devP4sswrd"
DB_NAME="vpiclist_lite1"
RUST_LOG="rvindecoder=info,tower_http=debug,axum::rejection=trace"
```
run the script or makefile target:
```shell
./load-data.sh
# or
make load_data
```
run the app
```shell
cargo run

   Compiling rvindecoder v0.1.0 (/home/jms/Projects/rvindecoder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.81s
     Running `target/debug/rvindecoder`
2024-12-22T20:15:21.388769Z  INFO rvindecoder: listening on 0.0.0.0:3000
```
if everything is ok, it can be tested using curl:
```shell
curl http://localhost:3000/decode/3FA6P0LU1KR1653572 | jq 
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  2691  100  2691    0     0   9177      0 --:--:-- --:--:-- --:--:--  9153
{
  "side_air_bag_locations": "1st and 2nd Rows",
  "bus_type": "Not Applicable",
  "electronic_stability_control_(esc)": "Standard",
  "trailer_body_type": "Not Applicable",
  "dynamic_brake_support_(dbs)": "Standard",
  "suggested_vin": "",
  "engine_brake_(hp)_from": "188",
  "wheel_size_front_(inches)": "17",
  "plant_city": "HERMOSILLO",
  "automatic_crash_notification_(acn)_/_advanced_automatic_crash_notification_(aacn)": "Standard",
  "body_class": "Sedan/Saloon",
  "model_year": "2019",
  "doors": "4",
  "crash_imminent_braking_(cib)": "Standard",
  "series": "SE Hybrid",
  "number_of_seats": "5",
  "bed_type": "Not Applicable",
  "trailer_type_connection": "Not Applicable",
  "anti_lock_braking_system_(abs)": "Standard",
  "knee_air_bag_locations": "1st Row (Driver and Passenger)",
  "custom_motorcycle_type": "Not Applicable",
  "plant_country": "MEXICO",
  "adaptive_driving_beam_(adb)": "Standard",
  "daytime_running_light_(drl)": "Standard",
  "wheel_size_rear_(inches)": "17",
  "bus_floor_configuration_type": "Not Applicable",
  "motorcycle_chassis_type": "Not Applicable",
  "steering_location": "Left-Hand Drive (LHD)",
  "model": "Fusion",
  "top_speed_(mph)": "155",
  "tire_pressure_monitoring_system_(tpms)_type": "Direct",
  "axles": "2",
  "auto_reverse_system_for_windows_and_sunroofs": "Standard",
  "keyless_ignition": "Standard",
  "wheel_base_(inches)_from": "112.20",
  "possible_values": "",
  "gross_vehicle_weight_rating_from": "Class 1: 6,000 lb or less (2,722 kg or less)",
  "blind_spot_warning_(bsw)": "Standard",
  "gross_vehicle_weight_rating_to": "Class 1: 6,000 lb or less (2,722 kg or less)",
  "lane_departure_warning_(ldw)": "Standard",
  "lane_keeping_assistance_(lka)": "Standard",
  "motorcycle_suspension_type": "Not Applicable",
  "other_engine_info": "4V AtK HEV",
  "error_code": "0",
  "semiautomatic_headlamp_beam_switching": "Standard",
  "error_text": "0 - VIN decoded clean. Check Digit (9th position) is correct",
  "engine_number_of_cylinders": "4",
  "vehicle_type": "PASSENGER CAR",
  "number_of_seat_rows": "2",
  "seat_belt_type": "Manual",
  "additional_error_text": "",
  "displacement_(l)": "2.0",
  "manufacturer_name": "FORD MOTOR COMPANY, MEXICO",
  "front_air_bag_locations": "1st Row (Driver and Passenger)",
  "cab_type": "Not Applicable",
  "fuel_type_primary": "Gasoline",
  "plant_state": "STATE OF MEXICO",
  "engine_manufacturer": "Ford",
  "adaptive_cruise_control_(acc)": "Standard",
  "pedestrian_automatic_emergency_braking_(paeb)": "Standard",
  "make": "FORD",
  "forward_collision_warning_(fcw)": "Standard",
  "displacement_(cc)": "2000.0",
  "electrification_level": "Strong HEV (Hybrid Electric Vehicle)",
  "displacement_(ci)": "122.04748818946",
  "vehicle_descriptor": "3FA6P0LU*KR",
  "fuel_type_secondary": "Electric",
  "traction_control": "Standard",
  "engine_configuration": "In-Line",
  "backup_camera": "Standard",
  "base_price_($)": "27555",
  "number_of_wheels": "4"
}
```

