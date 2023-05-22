#!/bin/bash

echo "$docker_password" | docker login --username "$docker_username" --password-stdin
docker push "$docker_username/$1:v0.1-${GIT_COMMIT::8}" 
docker push "$docker_username/$1:latest" &
wait