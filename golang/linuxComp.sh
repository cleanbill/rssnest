#!/bin/bash
echo "Compiling for linux"

echo "1. Set env vars"
export GOARCH=
export GOARM=
export GOOS=linux
export GOPATH=/home/mick/work/rssnest

echo "2. Building"
go build -o rssnest

