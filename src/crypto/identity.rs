//! Ed25519 identity management.

use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct Identity {
    pub signing_key: SigningKey,
}

impl Identity {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self { signing_key }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }
}
