#!/usr/bin/env bash

docker build -f automata.base.dockerfile -t kyrader/automata .
docker push kyrader/automata