#!/bin/bash

cd ./backend && ./build.sh decky && cd ..

cd ./src/usdpl_front && ./rebuild.sh decky && cd ../..

npm install && npm run build

unset USDPL_ENCRYPTION_KEY
