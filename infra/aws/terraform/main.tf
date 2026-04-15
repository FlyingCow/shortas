
# Create a VPC

resource "aws_vpc" "core_vpc" {
  cidr_block           = "10.1.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name = "${var.stage}-core-vpc"
  }
}

variable "public_subnet_cidrs" {
  type        = list(string)
  description = "Public Subnet CIDR values"
  default     = ["10.1.1.0/24", "10.1.2.0/24", "10.1.3.0/24"]
}

variable "private_subnet_cidrs" {
  type        = list(string)
  description = "Private Subnet CIDR values"
  default     = ["10.1.11.0/24", "10.1.12.0/24", "10.1.13.0/24"]
}

variable "azs" {
  type        = list(string)
  description = "Availability Zones"
  default     = ["us-east-1a", "us-east-1b", "us-east-1c"]
}



resource "aws_subnet" "core_public_subnets" {

  count                   = length(var.public_subnet_cidrs)
  vpc_id                  = aws_vpc.core_vpc.id
  cidr_block              = element(var.public_subnet_cidrs, count.index)
  map_public_ip_on_launch = true
  availability_zone       = element(var.azs, count.index)

  tags = {
    Name = "${var.stage}-core-public ${count.index + 1}"
  }
}

resource "aws_subnet" "core_private_subnets" {
  count             = length(var.private_subnet_cidrs)
  vpc_id            = aws_vpc.core_vpc.id
  cidr_block        = element(var.private_subnet_cidrs, count.index)
  availability_zone = element(var.azs, count.index)

  tags = {
    Name = "Private Subnet ${count.index + 1}"
  }
}

output "core_private_subnet_ids" {
  value = aws_subnet.core_private_subnets[*].id
}

output "core_public_subnet_ids" {
  value = aws_subnet.core_public_subnets[*].id
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
  route_table_id         = aws_route_table.core_public_route_table.id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.core_internet_gateway.id
}

resource "aws_route_table_association" "core_public_association" {
  count = length(var.public_subnet_cidrs)
  subnet_id      = element(aws_subnet.core_public_subnets[*].id, count.index)
  route_table_id = aws_route_table.core_public_route_table.id
}


resource "aws_security_group" "core_sg" {
  name        = "${var.stage}-core-sg"
  description = "Allow TLS inbound traffic"
  vpc_id      = aws_vpc.core_vpc.id

  ingress {
    description = "HTTP from VPC"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = [aws_vpc.core_vpc.cidr_block]
    #ipv6_cidr_blocks = [aws_vpc.core_vpc.ipv6_cidr_block]
  }

  ingress {
    description = "TLS from VPC"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = [aws_vpc.core_vpc.cidr_block]
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
