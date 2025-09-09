#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "# Clickhouse client"

if ! [ -x "$(command -v mongosh)" ]; then
    # Install prerequisite packages
    sudo apt-get install -y apt-transport-https ca-certificates curl gnupg

    # Download the ClickHouse GPG key and store it in the keyring
    curl -fsSL 'https://packages.clickhouse.com/rpm/lts/repodata/repomd.xml.key' | sudo gpg --dearmor -o /usr/share/keyrings/clickhouse-keyring.gpg

    # Get the system architecture
    ARCH=$(dpkg --print-architecture)

    # Add the ClickHouse repository to apt sources
    echo "deb [signed-by=/usr/share/keyrings/clickhouse-keyring.gpg arch=${ARCH}] https://packages.clickhouse.com/deb stable main" | sudo tee /etc/apt/sources.list.d/clickhouse.list

    # Update apt package lists
    sudo apt-get update

    sudo apt-get install -y clickhouse-client
fi


echo "# Mongodb tools"

if ! [ -x "$(command -v mongosh)" ]; then
    
    echo 'Mongosh is not installed - installing.' >&2
    
    sudo apt-get install gnupg
    wget -qO- https://www.mongodb.org/static/pgp/server-8.0.asc | sudo tee /etc/apt/trusted.gpg.d/server-8.0.asc
    echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu noble/mongodb-org/8.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-8.0.list
    sudo apt-get update
    sudo apt-get install -y mongodb-mongosh
fi


echo "# Fluvio tools"

if ! [ -x "$(command -v fluvio)" ]; then
    
    echo 'Fluvio is not installed - installing.' >&2
    
    curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
fi