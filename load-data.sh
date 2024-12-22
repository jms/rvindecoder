#!/usr/bin/env bash

set -e
DATA_URL="https://vpic.nhtsa.dot.gov/api/vPICList_lite_2024_11.bak.zip"
VIN_TMP_DIR="./vin_files"
MSSQL_SA_PASSWORD="devP4sswrd"

if [ ! -d "$VIN_TMP_DIR" ]
then
  mkdir "$VIN_TMP_DIR"
fi

docker compose up -d mssql

curl --output "$VIN_TMP_DIR/data.zip" $DATA_URL
unzip "$VIN_TMP_DIR/data.zip" -d "$VIN_TMP_DIR/"

BACKUP_FILE=$(find "$VIN_TMP_DIR" -name "*.bak")
docker compose cp "$BACKUP_FILE" mssql:/vindataset.bak

# sqlcmd tool location in the specific container image:
#   azure-sql-edge: /opt/mssql-tools/bin/sqlcmd
#   mssql-server: /opt/mssql-tools18/bin/sqlcmd

docker compose run --rm mssql /opt/mssql-tools/bin/sqlcmd -C -P "$MSSQL_SA_PASSWORD" -S mssql -U sa -Q \
"RESTORE DATABASE vpiclist_lite1 FROM DISK = '/vindataset.bak' WITH REPLACE, MOVE 'vpiclist_lite1' \
TO '/var/opt/mssql/data/vPICList_Lite.mdf', MOVE 'vpiclist_lite1_log' TO '/var/opt/mssql/data/vPICList_Lite_Log.ldf'"

rm -rf "$VIN_TMP_DIR"

