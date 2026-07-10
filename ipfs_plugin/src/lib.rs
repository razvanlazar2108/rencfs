use crate::crypto::write::CryptoWrite; // Rezolvă eroarea cu .finish()
use crate::crypto::{self, Cipher};
use shush_rs::SecretVec;
use std::io::{Cursor, Read, Write};

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

        // Folosim Cursor pentru a implementa Write + Seek + Read cerute de autor
        let memory_file = Cursor::new(Vec::new());
        let mut writer = crypto::create_write(memory_file, self.cipher, &self.key);

        writer
            .write_all(data)
            .map_err(|e| format!("Eroare la scrierea datelor IPFS: {:?}", e))?;

        // .finish() returnează obiectul intern (Cursor-ul) în caz de succes
        let finished_cursor = writer
            .finish()
            .map_err(|e| format!("Eroare la finalizarea criptării IPFS: {:?}", e))?;

        // Extriem vectorul de bytes din interiorul cursorului
        Ok(finished_cursor.into_inner())
    }

    /// Decriptează un bloc de date utilizând CryptoRead-ul nativ din rencfs
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
        // Generăm o cheie de test de 32 de bytes (pentru ChaCha20Poly1305)
        let test_key = vec![0u8; 32];
        let cipher = IpfsCipher::new(test_key);

        let original_data = b"Date secrete trimise prin IPFS cu CID unic!";

        // 1. Criptăm datele
        let encrypted = cipher.encrypt(original_data).expect("Criptarea a eșuat");
        assert_ne!(
            original_data.to_vec(),
            encrypted,
            "Datele criptate nu trebuie să fie la fel ca cele originale"
        );

        // 2. Decriptăm datele înapoi
        let decrypted = cipher.decrypt(&encrypted).expect("Decriptarea a eșuat");
        assert_eq!(
            original_data.to_vec(),
            decrypted,
            "Datele decriptate nu se potrivesc cu cele originale"
        );
    }
}
