# Use our base image that has Node, Rust, and Postgres deps installed already
FROM kyrader/automata:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the client application source code
COPY client /app/client

# Build the client application
RUN cd client && \
    npm install && \
    npm run build

# Copy the server source code
COPY server /app/server

# Build the server
RUN cd server && \
    cargo build --release --features embed_assets

# Start with a fresh image to reduce size
FROM alpine:latest
RUN apk update && \
    apk add --no-cache \
    ca-certificates libgcc libstdc++ libpq-dev postgresql-libs

WORKDIR /root/

COPY --from=builder /app/server/target/release/server .

# Expose the port the server listens on
EXPOSE 3000

# Set env vars for server configuration
ENV PORT=3000
ENV ADDRESS=0.0.0.0

# Set the server binary as the entry point
CMD ["./server"]