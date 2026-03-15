#!/bin/bash

cargo +nightly build --release && \
./elf2exe target/i586-rust_dos/release/lotus3-rs -o lotus3.exe && \
xxd -p lotus3.exe | tr -d '\n' | sed 's/66cf/cf90/gI' | xxd -r -p > lotus4.exe && \
~/src/dosbox ~/lotus/lotus4.exe
