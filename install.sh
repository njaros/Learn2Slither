#!/bin/sh

cargo build --bin game --release
mv target/release/game learn2slither.bin
