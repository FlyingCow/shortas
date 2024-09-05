TODO:
    - Add kubernetes terraform for the needed services
    - Add datadog vector as data aggregator
    - Add click tracker enriching hits data
    - Add click aggregator modeling and saving reports data 
    - Add aggregator api extracting reports data
    - Add router api to create/modify/delete links/settings data

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