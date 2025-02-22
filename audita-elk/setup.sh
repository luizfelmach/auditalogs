#!/bin/bash

mkdir -p data
touch data/access.log
docker compose up setup

echo "Run docker compose up -d"
