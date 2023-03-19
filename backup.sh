#!/bin/bash
# Get a list of all databases
databases=$(psql -t -c "SELECT datname FROM pg_database WHERE datistemplate = false;")
# Loop through each database and backup to a file
for db in $databases; do pg_dump $db > /root/backup/$db.sql
done
