#!/bin/bash
source .env
watch -n 0.1 curl -X GET "http://localhost:9200/_cat/indices/log-*" -u elastic:$ELASTIC_PASSWORD