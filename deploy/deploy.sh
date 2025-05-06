#!/bin/bash

docker volume remove auditalogs
docker volume create auditalogs
docker compose -f besu.yml up -d
docker compose -f elastic.yml up -d
docker compose -f app.yml up -d
docker compose -f monitoring.yml up -d
