FROM brendanburns/wasm-dev-base:0.0.1

# Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    /root/.cargo/bin/rustup target add wasm32-wasi



