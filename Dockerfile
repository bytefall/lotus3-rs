FROM rustlang/rust:nightly
    
WORKDIR /app

RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu && \
    rustup component add llvm-tools-preview && \
    cargo install cargo-binutils

# ENV RUSTFLAGS="-C link-arg=-Tlink.x"

# CMD cargo +nightly build --release -Zbuild-std=core,compiler_builtins,alloc -Zbuild-std-features=compiler-builtins-mem -Zjson-target-spec --target i586-rust_dos.json && \
#     cargo +nightly objcopy --release -- -O binary --binary-architecture=i386:x86 target/i586-rust_dos/release/lotus3.com

CMD cargo +nightly build --release -Z build-std=core,compiler_builtins,alloc -Z build-std-features=compiler-builtins-mem -Z json-target-spec --target i586-rust_dos.json
