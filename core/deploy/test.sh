#!/usr/bin/env bash
cd "$(dirname "$0")"

python ./add-certificate.local.py localhost ../redirect/flow-router/certs/cert.pem ../redirect/flow-router/certs/key.pem 
