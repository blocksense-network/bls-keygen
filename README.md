# keygen tools

Usage:

```
# Generate 10 BLS key-pairs
nix run github:blocksense-network/bls-keygen -- --count 10

# Print Anvil's default 10 EVM key-pairs
nix develop github:blocksense-network/bls-keygen -c anvil

# Generate EVM (secp256k1) key-pair
nix develop github:blocksense-network/bls-keygen -c cast wallet new
```
