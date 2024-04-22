use std::fs;
use std::io::Read;
use std::path::Path;

use crate::cli::TextSignFormat;
use crate::utils::get_data;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Verifier;
use ed25519_dalek::VerifyingKey;
use rand::rngs::OsRng;

trait TextSign {
    /// Sign the data from reader
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

trait TextVerify {
    /// verify the data from reader with signature
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool>;
}

trait TextLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>
    where
        Self: Sized;
}

struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into().unwrap();
        let blake3 = Self::new(key);
        Ok(blake3)
    }
}

impl TextLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>
    where
        Self: Sized,
    {
        let key = crate::process::process_genpass(32, true, true, true, true)?;
        Ok(vec![key.to_vec()])
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into().unwrap();
        let key = SigningKey::from_bytes(key);
        let ed25519_dalek = Self::new(key);
        Ok(ed25519_dalek)
    }
}

impl TextLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>
    where
        Self: Sized,
    {
        let sk = SigningKey::generate(&mut OsRng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into().unwrap();
        let key = VerifyingKey::from_bytes(key)?;
        let ed25519_dalek = Ed25519Verifier::new(key);
        Ok(ed25519_dalek)
    }
}

impl TextLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = get_data(input)?;
    let sign = match format {
        TextSignFormat::Blake3 => {
            let blake3 = Blake3::load(key)?;
            blake3.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let ed25519_dalek = Ed25519Signer::load(key)?;
            ed25519_dalek.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(sign);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> anyhow::Result<String> {
    let mut reader = get_data(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let blake3 = Blake3::load(key)?;
            blake3.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let ed25519_dalek = Ed25519Verifier::load(key)?;
            ed25519_dalek.verify(&mut reader, &sig)?
        }
    };
    if verified {
        Ok("Verified".to_string())
    } else {
        Ok("Not verified".to_string())
    }
}

pub fn process_generator(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> anyhow::Result<()> {
        let data = b"hello,world!";
        let blake3 = Blake3::load("fixture/blake3.txt")?;
        let sig = blake3.sign(&mut &data[..])?;
        assert!(blake3.verify(&mut &data[..], &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> anyhow::Result<()> {
        let data = b"hello,world!";
        let ed25519_dalek = Ed25519Signer::load("fixture/ed25519.sk")?;
        let sig = ed25519_dalek.sign(&mut &data[..])?;
        let ed25519_dalek = Ed25519Verifier::load("fixture/ed25519.pk")?;
        assert!(ed25519_dalek.verify(&mut &data[..], &sig)?);
        Ok(())
    }
}
