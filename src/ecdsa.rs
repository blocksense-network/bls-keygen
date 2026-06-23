use const_hex::encode_prefixed;
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::CryptoRngCore;
use tiny_keccak::{Hasher, Keccak};

use crate::Identity;

pub struct EthereumKeyPair {
    private_key: String,
    address: String,
}

impl Identity for EthereumKeyPair {
    fn public_part(&self) -> &str {
        &self.address
    }

    fn private_key(&self) -> &str {
        &self.private_key
    }

    fn key_type(&self) -> &str {
        "eth"
    }

    fn random(rng: &mut impl CryptoRngCore) -> Self {
        let signing_key = SigningKey::random(rng);
        let verifying_key = *signing_key.verifying_key();
        EthereumKeyPair {
            private_key: encode_prefixed(signing_key.to_bytes()),
            address: ethereum_address_from_public_key(&verifying_key),
        }
    }
}

fn ethereum_address_from_public_key(public_key: &VerifyingKey) -> String {
    let public_key_bytes = public_key.to_encoded_point(false);
    let public_key_bytes = &public_key_bytes.as_bytes()[1..]; // Remove the 0x04 prefix

    let mut hasher = Keccak::v256();
    hasher.update(public_key_bytes);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);

    encode_prefixed(&hash[12..])
}
