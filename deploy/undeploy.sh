#!/bin/bash

docker compose -f besu.yml down -v
docker compose -f elastic.yml down -v
docker compose -f app.yml down -v
docker compose -f monitoring.yml down -v
docker volume remove auditalogs
