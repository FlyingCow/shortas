
# Create a VPC
resource "aws_vpc" "core_vpc" {
  cidr_block = "10.1.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support = true

  tags = {
    Name = "${var.stage}-core-vpc"
  }
}

resource "aws_subnet" "core_public_subnet" {
    vpc_id = aws_vpc.core_vpc.id
    cidr_block = "10.1.1.0/24"
    map_public_ip_on_launch = true
    availability_zone = "us-east-1a"

  tags = {
    Name = "${var.stage}-core-public"
  }
}

resource "aws_internet_gateway" "core_internet_gateway" {
    vpc_id = aws_vpc.core_vpc.id

  tags = {
    Name = "${var.stage}-core-igw"
  }
}

resource "aws_route_table" "core_public_route_table" {
    vpc_id = aws_vpc.core_vpc.id

  tags = {
    Name = "${var.stage}-core-public-route-table"
  }
}

resource "aws_route" "core_public_default_route" {
  route_table_id = aws_route_table.core_public_route_table.id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id = aws_internet_gateway.core_internet_gateway.id
}

resource "aws_route_table_association" "core_public_association" {
  subnet_id = aws_subnet.core_public_subnet.id
  route_table_id = aws_route_table.core_public_route_table.id
}


resource "aws_security_group" "core_sg" {
  name        = "${var.stage}-core-sg"
  description = "Allow TLS inbound traffic"
  vpc_id      = aws_vpc.core_vpc.id

  ingress {
    description      = "HTTP from VPC"
    from_port        = 80
    to_port          = 80
    protocol         = "tcp"
    cidr_blocks      = [aws_vpc.core_vpc.cidr_block]
    #ipv6_cidr_blocks = [aws_vpc.core_vpc.ipv6_cidr_block]
  }

  ingress {
    description      = "TLS from VPC"
    from_port        = 443
    to_port          = 443
    protocol         = "tcp"
    cidr_blocks      = [aws_vpc.core_vpc.cidr_block]
    #ipv6_cidr_blocks = [aws_vpc.core_vpc.ipv6_cidr_block]
  }

  egress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }
}