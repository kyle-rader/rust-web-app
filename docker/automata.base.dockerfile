# Use a small Linux-based image as the base
FROM alpine:latest AS builder

# Set the working directory
WORKDIR /app

# Install necessary dependencies
RUN apk update && \
    apk add --no-cache \
    nodejs npm rust cargo pkgconfig openssl-dev libpq-dev

# Install Dev Tools
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

ENV PATH="/root/.cargo/bin:${PATH}"
