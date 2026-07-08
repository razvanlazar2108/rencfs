use crate::crypto::{self, Cipher};
use crate::crypto::write::CryptoWrite; // Rezolvă eroarea cu .finish()
use shush_rs::SecretVec;
use std::io::{Read, Write, Cursor};

pub struct IpfsCipher {
    key: SecretVec<u8>,
    cipher: Cipher,
}
