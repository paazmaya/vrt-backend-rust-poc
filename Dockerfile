# Use the official Rust image as the base image
# https://hub.docker.com/_/rust/tags?page=1&name=alpine
FROM rust:1.71-alpine3.18

# Set the working directory inside the container
WORKDIR /app

# Copy your project files into the container
# FIXME: Define specific files to reduce security risk
COPY . .

# Install required dependencies (e.g., PostgreSQL client libraries)
# https://pkgs.alpinelinux.org/packages?name=libpq
RUN apk add --update libpq musl-dev

# Build your Rust project
RUN cargo build

# Start your Rust application
CMD ["cargo", "run"]
