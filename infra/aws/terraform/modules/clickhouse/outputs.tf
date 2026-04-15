output "instance_id" {
  description = "EC2 instance ID"
  value       = aws_instance.clickhouse.id
}

output "private_ip" {
  description = "Private IP address"
  value       = aws_instance.clickhouse.private_ip
}

output "public_ip" {
  description = "Public IP address (if assigned)"
  value       = var.assign_elastic_ip ? aws_eip.clickhouse[0].public_ip : aws_instance.clickhouse.public_ip
}

output "http_endpoint" {
  description = "ClickHouse HTTP endpoint"
  value       = "http://${aws_instance.clickhouse.private_ip}:8123"
}

output "native_endpoint" {
  description = "ClickHouse native protocol endpoint"
  value       = "${aws_instance.clickhouse.private_ip}:9000"
}

output "data_volume_id" {
  description = "EBS data volume ID"
  value       = aws_ebs_volume.clickhouse_data.id
}
