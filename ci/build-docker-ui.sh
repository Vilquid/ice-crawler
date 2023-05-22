#!/bin/bash

[[ -z "${GIT_COMMIT}" ]] && Tag='local' || Tag="${GIT_COMMIT::8}"
[[ -z "${docker_username}" ]] && DockerRepo='' || DockerRepo="${docker_username}/"
docker build -t "${DockerRepo}$1:latest" -t "${DockerRepo}$1:v0.1-$Tag" icecrawlerui/