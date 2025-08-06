use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use clap::Parser;
use serde::Serialize;

use hex::encode;

use blst::min_pk::PublicKey;
use blst::min_pk::SecretKey;

/// BLS key generation utility
#[derive(Parser, Debug)]
#[command(name = "gen_bls_keys")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of keys to generate
    #[arg(short = 'c', long = "count", default_value = "1")]
    count: usize,

    /// Output directory for the generated keys
    #[arg(short = 'o', long = "output-dir", default_value = "./generated_keys")]
    output_dir: String,
}

#[derive(Serialize)]
struct PublicKeyEntry {
    id: usize,
    #[serde(rename = "bls-public-key")]
    bls_public_key: String,
}

fn main() {
    let args = Args::parse();

    let output_path = Path::new(&args.output_dir);
    let private_keys_dir = output_path.join("private-keys");
    fs::create_dir_all(&private_keys_dir).expect("Failed to create private-keys directory");

    let mut public_keys = Vec::new();

    for n in 0..args.count {
        // Generate keys for each iteration
        let (sk, pk) = generate_keys();
        let pk_hex = serialize_public_key(&pk);
        let sk_hex = serialize_priv_key(&sk);

        let private_key_path = private_keys_dir.join(format!("key{}", n + 1));
        let mut private_key_file =
            File::create(&private_key_path).expect("Failed to create private key file");
        writeln!(private_key_file, "{}", sk_hex).expect("Failed to write private key");

        public_keys.push(PublicKeyEntry {
            id: n + 1,
            bls_public_key: pk_hex,
        });
    }

    let public_keys_path = output_path.join("public-keys.json");
    let public_keys_file =
        File::create(public_keys_path).expect("Failed to create public-keys.json");
    serde_json::to_writer_pretty(public_keys_file, &public_keys)
        .expect("Failed to write to public-keys.json");

    println!("Generated {} key pairs.", args.count);
}

fn generate_random_array() -> [u8; 35] {
    let mut file = File::open("/dev/random").unwrap();
    let mut array = [0u8; 35];
    file.read_exact(&mut array).unwrap();

    array
}

fn generate_keys() -> (SecretKey, PublicKey) {
    let ikm: &[u8; 35] = &generate_random_array();
    let sk = SecretKey::key_gen(ikm, &[]).expect("Failed to generate secret key");
    let pk = sk.sk_to_pk();
    (sk, pk)
}

fn serialize_public_key(pk: &PublicKey) -> String {
    encode(pk.to_bytes())
}

fn serialize_priv_key(sk: &SecretKey) -> String {
    encode(sk.to_bytes())
}
