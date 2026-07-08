use crate::crypto::{self, Cipher};
use crate::crypto::write::CryptoWrite; // Rezolvă eroarea cu .finish()
use shush_rs::SecretVec;
use std::io::{Read, Write, Cursor};

pub struct IpfsCipher {
    key: SecretVec<u8>,
    cipher: Cipher,
}

impl IpfsCipher {
    /// Inițializează plugin-ul cu o cheie sigură și cipher-ul implicit
    pub fn new(secret_key: Vec<u8>) -> Self {
        Self {
            key: SecretVec::from(secret_key),
            cipher: Cipher::ChaCha20Poly1305,
        }
    }

    /// Criptează un bloc de date utilizând un Cursor pentru a simula un fișier în memorie
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        
    }
}