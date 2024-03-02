FROM rust:1.73-bookworm

# install wasm support
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add llvm-tools
RUN rustup component add clippy
RUN cargo install cargo-llvm-cov cargo-hack mdbook
