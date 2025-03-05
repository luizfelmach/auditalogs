#!/bin/bash

THREADS_WORKERS=(4)
THREADS_DISPATCHERS=(4)
BATCH_SIZE=(100 100000)

SERVER_CMD="audita-worker"
K6_SCRIPT="k6.js"

echo "metric_name,timestamp,metric_value,check,error,error_code,expected_response,group,method,name,proto,scenario,service,status,subproto,tls_version,url,extra_tags,metadata" > result.csv

for workers in "${THREADS_WORKERS[@]}"; do
  for dispatchers in "${THREADS_DISPATCHERS[@]}"; do
    for batch in "${BATCH_SIZE[@]}"; do

      echo "=============================================="
      echo "Testing configuration:"
      echo "THREADS_WORKERS: $workers"
      echo "THREADS_DISPATCHERS: $dispatchers"
      echo "BATCH_SIZE: $batch"
      echo "=============================================="

      THREADS_WORKERS=$workers \
      THREADS_DISPATCHERS=$dispatchers \
      BATCH_SIZE=$batch \
      $SERVER_CMD &

      SERVER_PID=$!

      sleep 3

      k6 run $K6_SCRIPT \
        --out csv=tmp.csv \
        --tag config="Workers: ${workers} Dispatchers: ${dispatchers} Batch: ${batch}"

      tail -n +2 tmp.csv >> result.csv
      rm tmp.csv

      kill $SERVER_PID
      wait $SERVER_PID 2>/dev/null

      sleep 1
    done
  done
done

echo "Todos os testes foram concluídos!"