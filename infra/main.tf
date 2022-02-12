terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.27"
    }
  }

  required_version = ">= 0.14.9"
}

variable "region" {
  type    = string
  default = "us-east-1"
}

provider "aws" {
  profile = "default"
  region  = var.region
}

# ----------------------------------------
# VPC
# ----------------------------------------

resource "aws_vpc" "main_vpc" {
  cidr_block = "10.0.0.0/16"
  tags = {
    Name = "main-vpc"
  }
}

# For all internet-exposed services, like NAT gateways and ALB instances
resource "aws_subnet" "main_public_subnet" {
  vpc_id     = aws_vpc.main_vpc.id
  cidr_block = "10.0.0.0/24"
  tags = {
    name = "main-public-subnet"
  }
}

# All private services, even if they are avaialble to the web, should be in the
# private subnet and only exposed through the resources provisioned in the
# public subnet. 
resource "aws_subnet" "main_private_subnet_1" {
  vpc_id     = aws_vpc.main_vpc.id
  cidr_block = "10.0.1.0/24"
  tags = {
    name = "main-private-subnet-1"
  }
}

# Need two private subnets because database subnet groups must be available in
# at least two AZs
resource "aws_subnet" "main_private_subnet_2" {
  vpc_id     = aws_vpc.main_vpc.id
  cidr_block = "10.0.2.0/24"
  tags = {
    name = "main-private-subnet-2"
  }
}

# ----------------------------------------
# IAM policies and groups
# ----------------------------------------

resource "aws_iam_policy" "admin" {
  name        = "admin-policy"
  path        = "/admin/"
  description = "No rules!"

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action   = "*"
        Effect   = "Allow"
        Resource = "*"
      },
    ]
  })
}

resource "aws_iam_group" "admins" {
  name = "admins"
  path = "/admins/"
}

resource "aws_iam_group_policy" "admins_policy" {
  name   = "admins-policy"
  group  = aws_iam_group.admins.name
  policy = aws_iam_policy.admin.policy
}

resource "aws_iam_group_membership" "admins_group_memberships" {
  name  = "admins-group-membership"
  group = aws_iam_group.admins.name
  users = [aws_iam_user.harry.name]
}

# ----------------------------------------
# IAM users
# ----------------------------------------

resource "aws_iam_user" "harry" {
  name = "harry"
  path = "/harry/"
}

# ----------------------------------------
# RDS
# ----------------------------------------

# This ensures that the database is provisioned within the private
# subnet to avoid accidentally exposing it to the internet.
# https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_VPC.WorkingWithRDSInstanceinaVPC.html
resource "aws_db_subnet_group" "main_db_subnet_group" {
  name        = "main-db-subnet-group"
  description = "Private database subnet"
  subnet_ids = [
    aws_subnet.main_private_subnet_1.id,
    aws_subnet.main_private_subnet_2.id,
  ]
}

resource "aws_db_instance" "main_db" {
  identifier     = "main-db"
  engine         = "mysql"
  engine_version = "8.0.27"

  instance_class    = "db.t3.micro"
  storage_type      = "gp2"
  allocated_storage = 20

  username = "admin"
  password = "6293333b601013be34ad39d7"
  port     = "3306"

  deletion_protection  = true
  db_subnet_group_name = aws_db_subnet_group.main_db_subnet_group.name
}

# ----------------------------------------
# Service deployment
# ----------------------------------------

resource "aws_ecr_repository" "main_ecr" {
  # repository URI should be used in packer definitions for docker-push postprocess step
  name = "main-image-repository"
}

# resource "aws_key_pair" "deployer" {
#   key_name   = "deployer-key"
#   public_key = "key-here"
# }

# Create ECS tasks in the default cluster Each "logical service" should have a
# ECS task definition and a corresponding ECS service definition. Should use
# DAEMON scheduling strategy, since we don't really need load balancing or high
# reliability. This only schedules one task.

# resource "aws_ecs_task_definition" "my_service_task" {
#   family                   = "my-service-task"
#   requires_compatibilities = ["EC2"]
#   container_definitions = jsonencode([
#     {
#       name      = "first"
#       image     = "my-ecr-image"
#       cpu       = 10  # Check what units
#       memory    = 512 # Check what units this is in 
#       essential = true
#       portMappings = [
#         {
#           containerPort = 80
#           hostPort      = 80
#         },
#       ]
#     },
#   ])
# }
# 
# resource "aws_ecs_service" "my_service_service" {
#   name                = "my-service-service"
#   task_definition     = aws_ecs_task_definition.my_service_task.arn
#   scheduling_strategy = "DAEMON" # Schedule at most one task at a time
# }