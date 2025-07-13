#!/usr/bin/env bash
set -e 

echo Building image...
docker build -f rustwebapp.base.dockerfile -t kyrader/rustwebapp .
echo ✅ 

echo Pushing image to https://hub.docker.com/r/kyrader/rustwebapp/tags
docker push kyrader/rustwebapp
echo ✅
