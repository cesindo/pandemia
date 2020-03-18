#!/usr/bin/env bash


export RUST_BACKTRACE=1
export RUST_LOG=pandemia=debug

cargo run --bin pandemia_server


