FROM rust:1.73-bookworm

# install wasm support
RUN rustup target add wasm32-unknown-unknown

# install components
RUN rustup component add llvm-tools
RUN rustup component add clippy
RUN rustup component add rustfmt

# install dependencies
RUN apt update && \
    apt install -y --no-install-recommends gcc cmake && \
    rm -rf /var/lib/apt/lists/*

# install tooling
RUN cargo install --locked --version 0.4.37 mdbook
RUN cargo install --locked --version 0.6.6 cargo-llvm-cov
RUN cargo install --locked --version 0.6.20 cargo-hack
RUN cargo install --locked --version 1.24.0 just
RUN cargo install --locked --version 0.29.1 cargo-semver-checks
