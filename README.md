# Circom CLI in Rust

Build

```sh
cargo build --release
```

Get circomlib

```sh
npm install circomlib
```

Setup

```sh
./target/release/nova-cli setup a.circom
```

Prove

```sh
./target/release/nova-cli prove a.pp a.pk a_input.json a_start_input.json
```

Verify

```sh
./target/release/nova-cli verify a.proof a.vk a_start_input.json 7
```

