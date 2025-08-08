use std::fs::{create_dir_all, write, File};
use std::io::Write as IoWrite;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Error, Ok, Result};
use clap::Parser;
use rand_core::CryptoRngCore;
use serde::Serialize;

use age::{
    ssh::Recipient as SshRecipient, x25519::Recipient as AgeRecipient, Encryptor, Recipient,
};

mod bls;
mod ecdsa;
use crate::bls::BlsKeyPair;
use crate::ecdsa::EthereumKeyPair;

/// BLS (BLS12-381) and ECSDA (secp256k1) keygen utility
#[derive(Parser, Debug)]
#[command(name = "gen_bls_keys")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of keys to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Output directory for the generated keys
    #[arg(short, long, default_value = "./generated_keys")]
    output_dir: PathBuf,

    /// Recipients' public keys (age or SSH) for encrypting private keys
    #[arg(short, long, value_name = "RECIPIENT")]
    recipients: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct KeyEntry {
    id: usize,
    bls_public_key: String,
    ethereum_address: String,
}

trait Identity {
    /// Returns the public part of the key (e.g., public key for BLS, address
    /// for Ethereum)
    fn public_part(&self) -> &str;

    /// Returns the private key as a string
    fn private_key(&self) -> &str;

    /// Returns the type of the key (e.g., "bls" or "eth")
    fn key_type(&self) -> &str;

    /// Generates a random key pair
    fn random(rng: &mut impl CryptoRngCore) -> Self where Self: Sized;
}

fn main() -> Result<()> {
    let args = Args::parse();

    let private_keys_dir = args.output_dir.join("private-keys");
    create_dir_all(&private_keys_dir).expect("Failed to create private-keys directory");

    let recipients = args
        .recipients
        .iter()
        .map(parse_age_recipient)
        .collect::<Result<Vec<_>>>()?;

    let mut rng = &mut rand::thread_rng();
    let mut public_keys = Vec::new();
    for id in 1..=args.count {
        let bls_key_pair = Box::new(BlsKeyPair::random(&mut rng));
        let eth_key_pair = Box::new(EthereumKeyPair::random(&mut rng));

        write_private_key(
            &private_keys_dir,
            id,
            bls_key_pair.as_ref(),
            recipients.as_slice(),
        );

        write_private_key(
            &private_keys_dir,
            id,
            eth_key_pair.as_ref(),
            recipients.as_slice(),
        );

        public_keys.push(KeyEntry {
            id,
            bls_public_key: (*bls_key_pair).public_part().to_owned(),
            ethereum_address: (*eth_key_pair).public_part().to_owned(),
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
    Ok(())
}

fn parse_age_recipient(s: impl AsRef<str>) -> Result<Box<dyn Recipient>> {
    AgeRecipient::from_str(s.as_ref())
        .map_err(Error::msg)
        .map(|r| Box::new(r) as Box<dyn Recipient>)
        .or_else(|_| {
            SshRecipient::from_str(s.as_ref())
                .map_err(|e| anyhow!("{:?}", e))
                .map(|r| Box::new(r) as Box<dyn Recipient>)
        })
}

fn write_private_key(dir: &PathBuf, id: usize, key: &dyn Identity, recipients: &[Box<dyn Recipient>]) {
    // devnet-reporter-01-bls-priv-key.age
    let mut filename = dir.join(format!("reporter-{:03}-{}-priv-key", id, key.key_type()));

    let private_key = key.private_key();

    if recipients.is_empty() {
        write(&filename, private_key).expect("Failed to write private key");
    } else {
        filename.set_extension("age");
        write_encrypted_file(filename, private_key, recipients)
            .expect("Failed to write encrypted private key");
    }
}

fn write_encrypted_file<Data>(
    path: PathBuf,
    data: Data,
    recipients: &[Box<dyn Recipient>],
) -> Result<()>
where
    Data: AsRef<[u8]>,
{
    let mut file = File::create(&path)?;
    let encryptor = Encryptor::with_recipients(recipients.iter().map(|r| r.as_ref()))?;
    let mut writer = encryptor.wrap_output(&mut file)?;

    writer.write_all(data.as_ref())?;
    writer.finish()?;
    Ok(())
}




