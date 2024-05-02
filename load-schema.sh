#!/bin/bash

source .env

docker exec -i postgres bash -c "
 psql -U $POSTGRES_USER -d $POSTGRES_DB -f /schema/users.sql &&
 psql -U $POSTGRES_USER -d $POSTGRES_DB -f /schema/folders.sql &&
 psql -U $POSTGRES_USER -d $POSTGRES_DB -f /schema/files.sql &&
 psql -U $POSTGRES_USER -d $POSTGRES_DB -f /schema/get_folder_tree.sql
"
