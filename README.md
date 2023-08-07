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
nova-cli --verbose setup examples/toy.circom
```

## Prove

This command takes public parameters, prover key, input, start public input and creates a proof.

```sh
nova-cli --verbose prove examples/toy.pp examples/toy.pk examples/input.json examples/start_input.json
```

## Verify

This command takes proof, verifier key, start public input, iteration count and verifies the proof.

```sh
nova-cli --verbose verify examples/toy.proof examples/toy.vk examples/start_input.json 5
```

