#!/usr/bin/env bash

set -eux -o pipefail

./generate_modules.sh

cargo run -p quick-protobuf --example pb_rs_example_v3_owned
cargo run -p quick-protobuf --example pb_rs_example
cargo run -p quick-protobuf --example pb_rs_example_v3
cargo run -p quick-protobuf --example pb_rs_example_nostd --no-default-features

cargo test -p pb-rs -p quick-protobuf --examples --tests
