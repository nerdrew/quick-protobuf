#!/usr/bin/env bash

set -eux -o pipefail

./generate_modules.sh

cargo test -p pb-rs -p quick-protobuf
