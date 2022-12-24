#!/bin/bash
# run docker container locally (for testing)
# assumes you're running in the backend/ dir of the project

docker run -i --entrypoint /caylon/backend/entrypoint-debug.sh -v $PWD/../:/caylon caylon_backend
mkdir -p ../bin
cp ./out/backend ../bin
