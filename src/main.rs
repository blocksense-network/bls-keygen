use std::fs::{create_dir_all, write};
use std::path::PathBuf;

use clap::Parser;
use const_hex::{encode, encode_prefixed};
use rand::RngCore;
use serde::Serialize;

use blst::min_pk::SecretKey as BlsPrivateKey;
use k256::ecdsa::{SigningKey as EcdsaPrivateKey, VerifyingKey as EcdsaPublicKey};
use tiny_keccak::{Hasher, Keccak};

/// BLS key generation utility
#[derive(Parser, Debug)]
#[command(name = "gen_bls_keys")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of keys to generate
    #[arg(short = 'c', long = "count", default_value_t = 1)]
    count: usize,

    /// Output directory for the generated keys
    #[arg(short = 'o', long = "output-dir", default_value = "./generated_keys")]
    output_dir: PathBuf,
}

#[derive(Serialize)]
struct KeyEntry {
    id: usize,

    #[serde(rename = "bls-public-key")]
    bls_public_key: String,

    #[serde(rename = "ethereum-address")]
    ethereum_address: String,
}

fn main() {
    let args = Args::parse();

    let private_keys_dir = args.output_dir.join("private-keys");
    create_dir_all(&private_keys_dir).expect("Failed to create private-keys directory");

    let mut public_keys = Vec::new();

    for n in 0..args.count {
        let bls_key_pair = generate_bls_key();
        let eth_key_pair = generate_ethereum_key();

        write(
            private_keys_dir.join(format!("key{}_bls", n + 1)),
            bls_key_pair.private_key,
        )
        .expect("Failed to write BLS private key");

        write(
            private_keys_dir.join(format!("key{}_eth", n + 1)),
            eth_key_pair.private_key,
        )
        .expect("Failed to write Ethereum private key");

        public_keys.push(KeyEntry {
            id: n + 1,
            bls_public_key: bls_key_pair.public_key,
            ethereum_address: eth_key_pair.address,
        });
    }

    write(
        args.output_dir.join("public-keys.json"),
        serde_json::to_string_pretty(&public_keys).expect("Failed to serialize public keys"),
    )
    .expect("Failed to write public-keys.json");

    println!(
        "Generated {} key pairs in {}",
        args.count,
        args.output_dir.display()
    );
}

struct BlsKeyPair {
    private_key: String,
    public_key: String,
}

fn generate_bls_key() -> BlsKeyPair {
    let mut ikm = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut ikm);

    let sk = BlsPrivateKey::key_gen(&ikm, &[]).expect("Failed to generate secret key");
    let pk = sk.sk_to_pk();

    BlsKeyPair {
        private_key: encode(sk.to_bytes()),
        public_key: encode(pk.to_bytes()),
    }
}

struct EthereumKeyPair {
    private_key: String,
    address: String,
}

fn generate_ethereum_key() -> EthereumKeyPair {
    let signing_key = EcdsaPrivateKey::random(&mut rand::thread_rng());
    let verifying_key = *signing_key.verifying_key();
    EthereumKeyPair {
        private_key: encode_prefixed(signing_key.to_bytes()),
        address: ethereum_address_from_public_key(&verifying_key),
    }
}

fn ethereum_address_from_public_key(public_key: &EcdsaPublicKey) -> String {
    let public_key_bytes = public_key.to_encoded_point(false);
    let public_key_bytes = &public_key_bytes.as_bytes()[1..]; // Remove the 0x04 prefix

    let mut hasher = Keccak::v256();
    hasher.update(public_key_bytes);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);

    encode_prefixed(&hash[12..])
}
