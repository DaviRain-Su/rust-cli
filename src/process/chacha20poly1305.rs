use crate::utils::get_data;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use std::fs;
use std::io::Read;
use std::path::Path;

pub struct ChaCha20Poly1305Algorithm {
    pub key: ChaCha20Poly1305,
}

impl ChaCha20Poly1305Algorithm {
    pub fn new(key: ChaCha20Poly1305) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let cipher = ChaCha20Poly1305::new_from_slice(key)?;
        let chacha20poly1305 = Self::new(cipher);
        Ok(chacha20poly1305)
    }

    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub fn process_text_encrypt(input: &str, key: &str) -> anyhow::Result<String> {
    let mut reader = get_data(input)?;
    let chacha20poly1305 = ChaCha20Poly1305Algorithm::load(key)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let nonce = Nonce::default();
    let ciphertext = chacha20poly1305
        .key
        .encrypt(&nonce, &*buf)
        .map_err(|e| anyhow::anyhow!(e))?;
    let signed = URL_SAFE_NO_PAD.encode(ciphertext);
    Ok(signed)
}

pub fn process_text_decrypt(input: &str, key: &str) -> anyhow::Result<String> {
    let mut reader = get_data(input)?;
    let chacha20poly1305 = ChaCha20Poly1305Algorithm::load(key)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let nonce = Nonce::default();
    let decode_ciphertext = URL_SAFE_NO_PAD.decode(buf)?;
    let plaintext = chacha20poly1305
        .key
        .decrypt(&nonce, decode_ciphertext.as_ref())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(String::from_utf8(plaintext)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha20poly1305_sign_verify() -> anyhow::Result<()> {
        // let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = ChaCha20Poly1305::new_from_slice(b"2w9#$w%&99hUbny8e_xqCL&Z$HrM3*AG").unwrap();
        // let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let nonce = Nonce::default(); // 96-bits; unique per message
        let ciphertext = cipher
            .encrypt(&nonce, b"plaintext message".as_ref())
            .unwrap();
        println!("ciphertext: {:?}", ciphertext);
        let signed = URL_SAFE_NO_PAD.encode(ciphertext.clone());
        println!("ciphertext base64 encode: {:?}", signed);
        let decode_ciphertext = URL_SAFE_NO_PAD.decode(signed.clone()).unwrap();
        println!("ciphertext base64 decode: {:?}", decode_ciphertext);
        assert_eq!(decode_ciphertext, ciphertext);
        let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
        println!("plaintext: {:?}", plaintext);
        assert_eq!(&plaintext, b"plaintext message");
        Ok(())
    }
}
