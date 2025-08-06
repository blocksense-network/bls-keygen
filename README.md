# BLS (BLS12-381) and ECDSA (secp256k1) keygen tools

## Usage

### Generate 3 BLS and ECDSA key-pairs in `./keys/`

```bash
nix run github:blocksense-network/bls-keygen -- --count 3 --output-dir ./keys/
```

### Output structure

```bash
tree -F --dirsfirst ./keys
./keys/
├── private-keys/
│   ├── key1_bls
│   ├── key1_eth
│   ├── key2_bls
│   ├── key2_eth
│   ├── key3_bls
│   └── key3_eth
└── public-keys.json
```

`keys/public-keys.json`:

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
  {
    "id": 3,
    "bls-public-key": "9702e7f34b26d710e339488e0496c1593269d41da4a77d07ce3a040b8b644e44bc956dd7a56cfc35bd1ca543e53c4d9e",
    "ethereum-address": "0xcd5b370d73f7b1e20277487ae750a81867b44546"
  }
]
```
