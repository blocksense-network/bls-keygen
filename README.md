# BLS (BLS12-381) and ECDSA (secp256k1) key generator

A CLI that generates pairs of BLS12-381 and Ethereum (secp256k1) identities.

## Output structure

All files are generated under `--output-dir $DIR` (by default `$DIR` is `./generated_keys/`)

- Private keys: `$DIR/private-keys/`
- JSON index of the public parts `$DIR/public-keys.json`

```text
$DIR/
├── private-keys/
│   ├── private_key_1_bls
│   ├── private_key_1_eth
│   ├── private_key_2_bls
│   ├── private_key_2_eth
│   └── ...
└── public-keys.json
```

To use the built-in
[`age`](https://github.com/str4d/rage/tree/main/age#age-rust-library)
encryption, you can pass `--recipients ...` with `age` or `ssh` recipients. The
encrypted private keys will have the `.age` extension.

```text
$DIR/
├── private-keys/
│   ├── private_key_1_bls.age
│   ├── private_key_1_eth.age
│   ├── private_key_2_bls.age
│   ├── private_key_2_eth.age
│   └── ...
└── public-keys.json
```

## Quick start

### Run with Nix (no checkout needed)

```sh
# Show help
nix run github:metacraft-labs/bls-keygen -- --help

# Generate 3 BLS & ECDSA key pairs
nix run github:metacraft-labs/bls-keygen -- --count 3

# Generate 20 and encrypt them with age recipient
nix run github:metacraft-labs/bls-keygen -- \
  --count 20 \
  --recipients 'age1znhknh35p5t5e65pag2quyw2tn52948x6k73pfc3k9a9htgak9hs2tx25m'
```

## CLI

Binary name: `bls-ecdsa-keygen`

Options:

- `-c, --count <N>`: number of key pairs to generate (default: 1).
- `-o, --output-dir <PATH>`: output directory (default: `./generated_keys`).
- `-r, --recipients <RECIPIENT>`: repeatable; public recipients to encrypt private keys with.
  - Accepts age X25519 recipients (e.g., `age1...`) and SSH recipients (e.g., `ssh-ed25519 AAAA... user@host`)

Examples:

- Plaintext keys:
  ```fish
  bls-ecdsa-keygen --count 2 --output-dir ./keys
  ```
- Encrypted keys with age and SSH recipients:
  ```fish
  bls-ecdsa-keygen --count 2 \
    --recipients 'age1exampleexampleexampleexampleexampleexampleexampleexampleqf3h' \
    --recipients 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFakeBase64HereOnlyAsExample user@host'
  ```


`public-keys.json` example:

```json
[
  {
    "id": 1,
    "bls-public-key": "8c69201ef402f2fba975505c0b5f928aa7010822604f28f24809eec810dbbb74b55ee6c55b7cd54f7184d51bfc856649",
    "ethereum-address": "0x1115e8385ebf2e74cd2585eb3d08ef5937d37300"
  },
  {
    "id": 2,
    "bls-public-key": "858ea040455ab85d9a3dc951b9b0942bd67fb549649cdc9dafcf544d79c5ef74e0597214756649e0b8760113b7ea3f24",
    "ethereum-address": "0xe3c33377f6c420970a4709122e603f73a38f0d3c"
  },
  ...
]
```

## Encrypting and decrypting private keys

If you use `--recipients`, private keys are written as age-encrypted files. Example usage:

Generate:

```sh
nix run github:blocksense-network/bls-keygen -- \
  --count 5 \
  --recipients $(cat ~/.ssh/id_ed25519.pub)
```

Decrypt:

```fish
rage \
  --decrypt ./generated_keys/private-keys/private_key_eth_1.age \
  -i ~/.ssh/id_ed25519
```

Note: files are binary age format (not ASCII-armored).

## Development

- Clone and enable the Nix dev shell:

  ```sh
  git clone https://github.com/metacraft-labs/bls-keygen.git
  cd bls-keygen
  direnv allow
  cargo build
  ```

- With direnv (auto-load dev shell on cd):

  ```fish
  # enable direnv for your shell once (fish)
  echo 'direnv hook fish | source' >> ~/.config/fish/config.fish
  direnv allow
  ```

- Build a runnable derivation with Nix:

  ```fish
  nix build
  ./result/bin/bls-ecdsa-keygen --help
  ```

## Implementation notes

- BLS keys are generated with `blst::min_pk`.
- Ethereum keys are generated with `k256` and addresses derived via Keccak-256 of the uncompressed public key (sans 0x04), using the last 20 bytes.
- Private keys can be encrypted to age using age X25519 or SSH recipients.

## Security

This tool is intended for development and provisioning workflows. Handle generated private keys with care. Ensure you trust your OS randomness source and avoid committing unencrypted private materials to VCS.
