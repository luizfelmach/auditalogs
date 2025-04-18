#!/bin/bash

docker compose -f besu.yml up -d
docker compose -f elastic.yml up -d
docker compose -f app.yml up -d
docker compose -f monitoring.yml up -d
