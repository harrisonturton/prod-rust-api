packer {
  required_plugins {
    docker = {
      version = "1.0.3"
      source  = "github.com/hashicorp/docker"
    }
  }
}

# ---------------------------------------------
# Database config
# ---------------------------------------------

variable "postgres_user" {
  type    = string
  default = "postgres"
}

variable "postgres_password" {
  type    = string
  default = "password"
}

variable "postgres_host" {
  type    = string
  default = "localhost"
}

variable "postgres_port" {
  type    = string
  default = "5432"
}

# ---------------------------------------------
# Image config
# ---------------------------------------------

variable "docker_image" {
  type    = string
  default = "rust:1.56.0"
}

source "docker" "rust" {
  image  = var.docker_image
  commit = true
  changes = [
    "ENTRYPOINT ./app/target/release/backend"
  ]
}

build {
  name    = "backend"
  sources = ["source.docker.rust"]

  provisioner "shell" {
    environment_vars = [
      "DB_USER=${var.postgres_user}",
      "DB_PASSWORD=${var.postgres_password}",
      "DB_HOST=${var.postgres_host}",
      "DB_PORT=${var.postgres_port}",
    ]
    inline = [
      "echo Provisioning with ${var.docker_image} image",
      "echo DB_USER is $DB_USER",
      "cargo --version",
      "mkdir /app",
    ]
  }

  provisioner "file" {
    source      = "../../src"
    destination = "/app"
  }

  provisioner "file" {
    source      = "../../Cargo.toml"
    destination = "/app/"
  }

  provisioner "shell" {
    inline = [
      "cd /app",
      "cargo build --release",
    ]
  }

  post-processor "docker-tag" {
    repository = "backend"
  }
}