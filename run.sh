#!/bin/bash

set -e

TEMP_DB=$(pg_tmp -w 600)
echo $TEMP_DB
DATABASE_URL=$TEMP_DB diesel migration run
DATABASE_URL=$TEMP_DB cargo run
