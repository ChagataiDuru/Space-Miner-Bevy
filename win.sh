#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/space_miner.exe . &&
exec ./space_miner.exe "$@"
