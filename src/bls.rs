use const_hex::encode;
use rand_core::CryptoRngCore;

use blst::min_pk::SecretKey;

use crate::Identity;

pub struct BlsKeyPair {
    private_key: String,
    public_key: String,
}

pub const MULTIFORMATS_BLS_PUBKYE_PREFIX: &str = "ea30";

impl Identity for BlsKeyPair {
    fn private_key(&self) -> &str {
        &self.private_key
    }

    fn public_part(&self) -> &str {
        &self.public_key
    }

    fn key_type(&self) -> &str {
        "bls"
    }

    fn random(rng: &mut impl CryptoRngCore) -> Self {
        let mut ikm = [0u8; 64];
        rng.fill_bytes(&mut ikm);

        let sk = SecretKey::key_gen(&ikm, &[]).expect("Failed to generate secret key");
        let pk = sk.sk_to_pk();

        BlsKeyPair {
            private_key: encode(sk.to_bytes()),
            public_key: format!("{MULTIFORMATS_BLS_PUBKYE_PREFIX}{0}", encode(pk.to_bytes())),
        }
    }
}
