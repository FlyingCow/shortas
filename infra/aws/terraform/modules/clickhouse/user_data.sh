#!/bin/bash
set -e

# Update system
yum update -y

# Install required packages
yum install -y yum-utils device-mapper-persistent-data lvm2

# Add ClickHouse repository
yum-config-manager --add-repo https://packages.clickhouse.com/rpm/clickhouse.repo

# Install ClickHouse
yum install -y clickhouse-server clickhouse-client

# Wait for data volume to be attached
while [ ! -e ${data_volume_device} ]; do
  echo "Waiting for data volume..."
  sleep 5
done

# Format and mount data volume (only if not already formatted)
if ! blkid ${data_volume_device}; then
  mkfs -t ext4 ${data_volume_device}
fi

mkdir -p /var/lib/clickhouse
mount ${data_volume_device} /var/lib/clickhouse

# Add to fstab for persistence
echo "${data_volume_device} /var/lib/clickhouse ext4 defaults,nofail 0 2" >> /etc/fstab

# Set permissions
chown -R clickhouse:clickhouse /var/lib/clickhouse

# Configure ClickHouse
cat > /etc/clickhouse-server/config.d/custom.xml <<EOF
<clickhouse>
    <logger>
        <level>information</level>
        <log>/var/log/clickhouse-server/clickhouse-server.log</log>
        <errorlog>/var/log/clickhouse-server/clickhouse-server.err.log</errorlog>
        <size>100M</size>
        <count>10</count>
    </logger>

    <http_port>8123</http_port>
    <tcp_port>9000</tcp_port>

    <listen_host>0.0.0.0</listen_host>

    <max_connections>4096</max_connections>
    <keep_alive_timeout>3</keep_alive_timeout>
    <max_concurrent_queries>100</max_concurrent_queries>

    <uncompressed_cache_size>8589934592</uncompressed_cache_size>
    <mark_cache_size>5368709120</mark_cache_size>

    <path>/var/lib/clickhouse/</path>
    <tmp_path>/var/lib/clickhouse/tmp/</tmp_path>
    <user_files_path>/var/lib/clickhouse/user_files/</user_files_path>

    <users_config>users.xml</users_config>
    <default_profile>default</default_profile>
    <default_database>default</default_database>
</clickhouse>
EOF

# Configure users with password
cat > /etc/clickhouse-server/users.d/custom.xml <<EOF
<clickhouse>
    <users>
        <default>
            <password>${clickhouse_password}</password>
            <networks>
                <ip>::/0</ip>
            </networks>
            <profile>default</profile>
            <quota>default</quota>
            <access_management>1</access_management>
        </default>
    </users>
</clickhouse>
EOF

# Start ClickHouse
systemctl enable clickhouse-server
systemctl start clickhouse-server

# Wait for ClickHouse to start
sleep 10

# Create database and tables
clickhouse-client --password="${clickhouse_password}" --query="CREATE DATABASE IF NOT EXISTS shortas"

# Create clicks table for analytics
clickhouse-client --password="${clickhouse_password}" --query="
CREATE TABLE IF NOT EXISTS shortas.clicks (
    timestamp DateTime,
    link_id String,
    owner_id String,
    hostname String,
    country String,
    device_type String,
    browser String,
    os String,
    referrer String,
    is_bot UInt8,
    response_time_ms UInt32
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (owner_id, link_id, timestamp)
TTL timestamp + INTERVAL 365 DAY
"

# Create aggregated stats table
clickhouse-client --password="${clickhouse_password}" --query="
CREATE TABLE IF NOT EXISTS shortas.clicks_hourly (
    hour DateTime,
    link_id String,
    owner_id String,
    hostname String,
    country String,
    device_type String,
    clicks UInt64,
    unique_clicks UInt64
) ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(hour)
ORDER BY (owner_id, link_id, hour, country, device_type)
"

# Install CloudWatch agent for metrics
yum install -y amazon-cloudwatch-agent

cat > /opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json <<EOF
{
    "agent": {
        "metrics_collection_interval": 60,
        "run_as_user": "root"
    },
    "metrics": {
        "namespace": "CWAgent",
        "metrics_collected": {
            "cpu": {
                "measurement": ["cpu_usage_active"],
                "metrics_collection_interval": 60,
                "totalcpu": true
            },
            "disk": {
                "measurement": ["disk_used_percent"],
                "metrics_collection_interval": 60,
                "resources": ["/var/lib/clickhouse"]
            },
            "mem": {
                "measurement": ["mem_used_percent"],
                "metrics_collection_interval": 60
            }
        }
    }
}
EOF

systemctl enable amazon-cloudwatch-agent
systemctl start amazon-cloudwatch-agent

# Setup backup cron job
cat > /etc/cron.daily/clickhouse-backup <<EOF
#!/bin/bash
BACKUP_NAME=\$(date +%Y%m%d_%H%M%S)
clickhouse-client --password="${clickhouse_password}" --query="BACKUP DATABASE shortas TO S3('s3://${s3_backup_bucket}/clickhouse/\$BACKUP_NAME', '${region}')"
EOF
chmod +x /etc/cron.daily/clickhouse-backup

echo "ClickHouse setup complete"
