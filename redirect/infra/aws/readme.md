Manual setup
1. add 127.0.0.1 kafka to /etc/hosts
sudo nano /etc/hosts

Useful command
kafka console consumer:
~/dev/kafka/bin$ ./kafka-console-consumer.sh --bootstrap-server kafka:9092  --topic hit-stream-local --property "print.key=true"