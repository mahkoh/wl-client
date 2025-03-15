#!/bin/bash

set -ex

cd "$(dirname "$0")"

function build() {
  rm -rf $1
  cargo run --bin wl-client-builder -- --xml-dir $2 --wl-client-path $3 $1
  rustfmt +nightly --edition 2024 $1/mod.rs
}

build ../wl-client/src/test_protocols        test_protocols    crate
build ../wl-client/src/protocols             protocols         crate
build ../wl-client/examples/common/protocols example_protocols ::wl_client
