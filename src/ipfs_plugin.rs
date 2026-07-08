use crate::crypto::{self, Cipher};
use crate::crypto::write::CryptoWrite; // Rezolvă eroarea cu .finish()
use shush_rs::SecretVec;
use std::io::{Read, Write, Cursor};