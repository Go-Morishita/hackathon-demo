#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

if [ "$#" -ne 6 ]; then
  echo "Usage: $0 <db_host> <db_port> <db_name> <username> <password> <sqlx_cmd>"
  exit 1
fi

db_host=$1
db_port=$2
db_name=$3
db_username=$4

# password can be a file path or literal
if [ -f "$5" ]; then
  db_password=$(<"$5")
else
  db_password=$5
fi

sqlx_cmd=$6

echo "Waiting for MySQL at ${db_host}:${db_port}..."
until mysql -h "$db_host" -P "$db_port" -u "$db_username" -p"$db_password" -e "USE $db_name;" &> /dev/null; do
  >&2 echo "MySQL is unavailable, sleeping"
  sleep 2
done

echo "MySQL is up - executing: $sqlx_cmd"
exec $sqlx_cmd
