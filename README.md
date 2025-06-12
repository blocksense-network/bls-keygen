# keygen tools

## Usage:

### Generate 10 BLS key-pairs

```bash
nix run github:blocksense-network/bls-keygen -- --count 10
```

### Print Anvil's default 10 EVM key-pairs

```bash
nix develop github:blocksense-network/bls-keygen -c anvil
```

### Generate EVM (secp256k1) key-pair

```bash
nix develop github:blocksense-network/bls-keygen -c cast wallet new
```
