use blst::min_pk::PublicKey;
use blst::min_pk::SecretKey;
use hex::encode;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    // Step 1: Generate keys
    let (sk, pk) = generate_keys();
    let mut keys_count = 1;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-c" | "--count" => {
                keys_count = args.next().expect("Value expected for count!").parse().expect("Int value needed!");
            }
            "--help" => {
                println!(
                    "Usage:
gen_bls_keys [options] [args]

OPTIONS
  --help                     show list of command-line options
  -c, --count                specify sequencer's config file path"
                );
                return;
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Unkown argument {}", arg);
                } else {
                    println!("Unkown positional argument {}", arg);
                }
            }
        }
    }

    for n in 0..keys_count {
        let pk_hex = serialize_public_key(&pk);
        println!("{}", n);
        println!("----------------------------------------------------------------------------------------------------------------");
        println!("Secret key (hex): {}", serialize_priv_key(&sk));
        println!("Public key (hex): {}", pk_hex);
    }

}

pub fn to_hex_string(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}

pub fn to_hex_string_p(mut bytes: Vec<u8>, padding_to: Option<usize>) -> String {
    if let Some(p) = padding_to {
        bytes.resize(p, 0);
    }
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
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
