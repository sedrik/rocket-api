#!/bin/bash

set -e

TEMP_DB=$(pg_tmp)
echo $TEMP_DB
DATABASE_URL=$TEMP_DB diesel migration run
DATABASE_URL=$TEMP_DB cargo test
DATABASE_URL=$TEMP_DB cargo doc --open
