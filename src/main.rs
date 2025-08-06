use std::fs::File;
use std::io::Read;

use clap::Parser;

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
}

fn main() {
    let args = Args::parse();

    for n in 0..args.count {
        // Generate keys for each iteration
        let (sk, pk) = generate_keys();
        let pk_hex = serialize_public_key(&pk);

        println!("{}", n);
        println!("----------------------------------------------------------------------------------------------------------------");
        println!("Secret key (hex): {}", serialize_priv_key(&sk));
        println!("Public key (hex): {}", pk_hex);
    }
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
