FROM kyrader/rustwebapp:latest

# This should be mapped as a volume in the docker-compose file
WORKDIR /app

# Run diesel setup and start the application with live-reload
CMD sleep 2 && diesel setup && cargo run live-reload
