#!/bin/bash

read -p "Índice do documento: " INDICE
read -p "ID do documento: " ID
read -p "Nome do novo campo: " NOVO_CAMPO
read -p "Valor do novo campo: " VALOR

ES_HOST="http://localhost:9200"

JSON_PAYLOAD=$(cat <<EOF
{
  "doc": {
    "$NOVO_CAMPO": "$VALOR"
  }
}
EOF
)

curl -X POST "$ES_HOST/$INDICE/_update/$ID" \
  -H 'Content-Type: application/json' \
  -u elastic:changeme \
  -d "$JSON_PAYLOAD"

