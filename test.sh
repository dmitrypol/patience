cargo test --features enable-system-alloc
cargo build
valkey-server --loadmodule target/debug/libpatience.dylib
# valkey-benchmark -q