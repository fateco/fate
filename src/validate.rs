use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, Signature, SignatureError, VerifyingKey};
use hex::{FromHex, FromHexError};
use thiserror::Error;
use worker::Env;

#[derive(Error, Debug)]
pub enum ValidationBuilderError {
    #[error("failed to decode hex string")]
    HexDecode(#[from] FromHexError),
    #[error("invalid cryptographic key bytes")]
    InvalidKeyBytes(#[from] SignatureError),
    #[error("missing secret or header processing error")]
    Worker(#[from] worker::Error),
}

pub fn public_key(env: &Env) -> Result<VerifyingKey, ValidationBuilderError> {
    Ok(VerifyingKey::from_bytes(
        &<[u8; PUBLIC_KEY_LENGTH]>::from_hex(env.secret("DISCORD_APP_PUBLIC_KEY")?.to_string())?,
    )?)
}

pub fn signature(headers: &worker::Headers) -> Result<Signature, ValidationBuilderError> {
    Ok(Signature::from_bytes(&<[u8; SIGNATURE_LENGTH]>::from_hex(
        headers.get("x-signature-ed25519")?.unwrap_or_default(),
    )?))
}

pub fn msg(
    body_bytes: &[u8],
    headers: &worker::Headers,
) -> Result<Vec<u8>, ValidationBuilderError> {
    let timestamp = headers.get("x-signature-timestamp")?.unwrap_or_default();
    Ok([timestamp.as_bytes(), body_bytes].concat())
}
