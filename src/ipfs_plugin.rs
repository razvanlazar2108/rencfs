pub struct IpfsCipher {
    // Stocăm cheia de criptare mai târziu
}

impl IpfsCipher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        Ok(data.to_vec())
    }
}
