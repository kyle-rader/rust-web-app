FROM kyrader/automata:latest

# This should be mapped as a volume in the docker-compose file
WORKDIR /app

CMD npm install && npm run start