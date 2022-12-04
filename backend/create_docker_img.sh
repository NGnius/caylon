#!/bin/bash
# build docker container locally (for testing)

cd .. && docker build -t caylon_backend -f ./backend/Dockerfile . && cd ./backend
