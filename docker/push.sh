#!/usr/bin/env bash
set -e 

echo Building image...
docker build -f automata.base.dockerfile -t kyrader/automata .
echo ✅ 

echo Pushing image to https://hub.docker.com/r/kyrader/automata/tags
docker push kyrader/automata
echo ✅
