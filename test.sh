#!/bin/bash

LISTEN_PORT=8080
CONNECT_PORT=8081
HOST="127.0.0.1"
SIZE=10G

nc -lkv $LISTEN_PORT | pv -prat -s $SIZE > /dev/null &
LISTENER_PID=$!

sleep 1
head -c $SIZE /dev/zero | nc -vN $HOST $CONNECT_PORT &
SENDER_PID=$!

echo "Sending $SIZE of data..."

trap 'kill $(jobs -p) 2>/dev/null; echo ""' EXIT
wait $SENDER_PID