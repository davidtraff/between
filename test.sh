#!/bin/bash

LISTEN_PORT=8080
CONNECT_PORT=8081
HOST="127.0.0.1"
SIZE=1G

nc -lkv $LISTEN_PORT | pv -prat -s $SIZE > /dev/null &
LISTENER_PID=$!

sleep 1
head -c $SIZE /dev/urandom | nc -vN $HOST $CONNECT_PORT &
SENDER_PID=$!

echo "Sending $SIZE of data..."

trap 'kill $(jobs -p) 2>/dev/null' EXIT
wait $SENDER_PID