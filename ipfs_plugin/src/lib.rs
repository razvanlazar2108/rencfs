use crate::crypto::write::CryptoWrite; // Resolves the error with .finish()
use crate::crypto::{self, Cipher};
use shush_rs::SecretVec;
use std::io::{Cursor, Read, Write};

pub struct IpfsCipher {
    key: SecretVec<u8>,
    cipher: Cipher,
}

impl IpfsCipher {
    /// Initializes the plugin with a secure key and the default cipher
    pub fn new(secret_key: Vec<u8>) -> Self {
        Self {
            key: SecretVec::from(secret_key),
            cipher: Cipher::ChaCha20Poly1305,
        }
    }

    /// Encrypts a block of data using a Cursor to simulate an in-memory file
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        // Use Cursor to implement Write + Seek + Read as required by the author
        let memory_file = Cursor::new(Vec::new());
        let mut writer = crypto::create_write(memory_file, self.cipher, &self.key);

        writer
            .write_all(data)
            .map_err(|e| format!("Eroare la scrierea datelor IPFS: {:?}", e))?;

        // .finish() returns the internal object (the Cursor) on success
        let finished_cursor = writer
            .finish()
            .map_err(|e| format!("Eroare la finalizarea criptării IPFS: {:?}", e))?;

        // Extract the byte vector from the inner cursor
        Ok(finished_cursor.into_inner())
    }

    /// Decrypts a block of data using the native CryptoRead from rencfs
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
        if encrypted_data.is_empty() {
            return Ok(Vec::new());
        }

        let mut reader = crypto::create_read(encrypted_data, self.cipher, &self.key);
        let mut decrypted_data = Vec::new();

        reader
            .read_to_end(&mut decrypted_data)
            .map_err(|e| format!("Eroare la decriptarea datelor IPFS: {:?}", e))?;

        Ok(decrypted_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipfs_encryption_decryption() {
        // Generate a 32-byte test key (required for ChaCha20Poly1305)
        let test_key = vec![0u8; 32];
        let cipher = IpfsCipher::new(test_key);

        let original_data = b"Date secrete trimise prin IPFS cu CID unic!";

        // 1. Encrypt the data
        let encrypted = cipher.encrypt(original_data).expect("Criptarea a eșuat");
        assert_ne!(
            original_data.to_vec(),
            encrypted,
            "Datele criptate nu trebuie să fie la fel ca cele originale"
        );

        // 2. Decrypt the data back
        let decrypted = cipher.decrypt(&encrypted).expect("Decriptarea a eșuat");
        assert_eq!(
            original_data.to_vec(),
            decrypted,
            "Datele decriptate nu se potrivesc cu cele originale"
        );
    }
}
