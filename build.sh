#!/bin/bash

cargo +nightly build --release && \
./elf2exe target/i586-rust_dos/release/lotus3-rs -o lotus3.exe && \
~/src/dosbox ~/lotus/lotus3.exe
