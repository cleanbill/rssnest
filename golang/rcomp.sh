#!/bin/bash
echo "Compiling for the raspberry pi"

echo "1. Set env vars"
export GOARCH=arm
export GOARM=5
export GOOS=linux
export GOPATH=/home/mick/work/rssnest

echo "2. Building"
go build -o rssnest 

echo "6. scp on to pi"
scp rssnest osmc@osmc:./rssnest/.
#scp conf.json osmc@osmc:./rssnest/.
#scp ./data/casts.json osmc@osmc:./rssnest/data/.
