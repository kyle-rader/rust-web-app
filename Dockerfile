# Use a small Linux-based image as the base
FROM alpine:latest as builder

# Set the working directory
WORKDIR /app

# Install necessary dependencies
RUN apk update && \
  apk add --no-cache nodejs npm rust cargo

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
  cargo build --release

# Start fresh image to reduce size
FROM alpine:latest
RUN apk --no-cache add ca-certificates libgcc libstdc++

WORKDIR /root/

COPY --from=builder /app/server/target/release/server .

# Expose the port the server listens on
EXPOSE 3000

# Set env vars for server configuration
ENV PORT=3000
ENV ADDRESS=0.0.0.0

# Set the server binary as the entry point
CMD ["./server"]