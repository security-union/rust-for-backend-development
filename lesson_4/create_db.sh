#!/usr/bin/env bash

dbmate wait

dbmate up

echo "Done"

exec "$@"
