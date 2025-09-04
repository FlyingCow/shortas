TODO:
    JWT authorization for api

    Click Router
        - Add SSL support (key storage, ACME http verification)
        - Deep links
        - API to create/modify/delete links/settings data

    Tracker
        - Click source detection
        - Data centers detection
        - Bot detection
    
    Aggs
        - API (reports, click stream)
        - S3 support for clickhouse

    SSL Bot
        - Domains Monitoring


HOW TO:

    Build
    - make build
     
    Run tracker
    - ./click-tracker -r development

    Run router
    - export AWS_ACCESS_KEY_ID=foobar
    - export AWS_SECRET_ACCESS_KEY=foobar
    - export AWS_DEFAULT_REGION=us-east-1
    - ./click-router -r development

    Manual setup
    1. add 127.0.0.1 kafka to /etc/hosts
    1. add 127.0.0.1 clickhouse to /etc/hosts
    sudo nano /etc/hosts

    Useful command
    kafka console consumer:
    ~/dev/kafka/bin$ ./kafka-console-consumer.sh --bootstrap-server kafka:9092  --topic hit-stream-local --property "print.key=true"