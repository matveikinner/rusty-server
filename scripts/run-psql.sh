#!/bin/bash
docker run --name rusty-database \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_DB=postgres \
  -v $PWD/scripts/init.sql:/docker-entrypoint-initdb.d/init.sql \
  -d postgres:15-alpine