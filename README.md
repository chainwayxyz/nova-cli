# Circom CLI in Rust

Build

```sh
cargo build --release
```

Install the binary

```sh
cargo install --path .
```

## Setup

This command takes a circom file and creates public parameters, prover key, verifier key.

```sh
nova-cli --verbose setup a.circom
```

## Prove

This command takes public parameters, prover key, input, start public input and creates a proof.

```sh
nova-cli --verbose prove a.pp a.pk a_input.json a_start_input.json
```

## Verify

This command takes proof, verifier key, start public input, iteration count and verifies the proof.

```sh
nova-cli --verbose verify a.proof a.vk a_start_input.json 7
```

