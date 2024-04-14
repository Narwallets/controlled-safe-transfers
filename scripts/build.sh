export RUSTFLAGS='-C link-arg=-s' 
cargo build -p controlled-transfer-contract --target wasm32-unknown-unknown --release
cargo build -p test-nep141-token --target wasm32-unknown-unknown --release
