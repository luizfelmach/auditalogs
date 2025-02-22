#!/bin/bash

generate_random_ip() {
  echo "$((RANDOM % 256)).$((RANDOM % 256)).$((RANDOM % 256)).$((RANDOM % 256))"
}

generate_random_mac() {
  echo "00:$(printf '%02x' $((RANDOM % 256))):$(printf '%02x' $((RANDOM % 256))):$(printf '%02x' $((RANDOM % 256))):$(printf '%02x' $((RANDOM % 256))):$(printf '%02x' $((RANDOM % 256)))"
}

generate_log() {
  ip=$(generate_random_ip)
  port=$((RANDOM % 65536))
  timestamp=$(LC_TIME=en_US.UTF-8 date +'%d/%b/%Y:%H:%M:%S %z')
  mac=$(generate_random_mac)
  log="$ip - - [$timestamp] Port: $port, MAC: $mac"
  echo "$log"
}

generate_logs_infinitely() {
  while true; do
    generate_log >> data/access.log
    sleep $(awk -v min=0.01 -v max=1 'BEGIN{srand(); print min+rand()*(max-min)}')  # Random sleep time between 0.1 and 1 second
  done
}

generate_logs_infinitely
