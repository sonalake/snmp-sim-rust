#!/bin/sh
docker-compose down
docker system prune --force
docker volume prune --force

docker-compose pull
docker-compose build

docker-compose up -d
docker-compose ps

