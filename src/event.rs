use serde::{Serialize, Deserialize};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub agent_id: String,
    pub payload: String,
    pub signature: Vec<u8>,
}

pub fn sign_event(keypair: &Keypair, payload: &str) -> Vec<u8> {
    keypair.sign(payload.as_bytes()).to_bytes().to_vec()
}

pub fn verify_event(public_key: &PublicKey, event: &Event) -> bool {
    public_key.verify(event.payload.as_bytes(), &Signature::from_bytes(&event.signature).unwrap()).is_ok()
}