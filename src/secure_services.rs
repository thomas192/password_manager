use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, BlockDecryptMut, KeyIvInit};

mod services;
use services::Services;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

static VAULT_PATH: &str = "vault.json";

pub struct SecureServices {
    services: Services,
    key: [u8; 32],
}

impl SecureServices {
    pub fn create(password: String) -> Result<(), Box<dyn Error>> {
        SecureServices { 
            services: Services::new(vec![]),
            key: SecureServices::gen_key(password),
        }.store()
    }
    
    pub fn load(password: String) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(VAULT_PATH);

        if !path.exists() {
            return Err("vault file does not exist".into());
        }
        
        let key = SecureServices::gen_key(password);
        let ciphertext = std::fs::read(&path)?;
        let mut buf = vec![0u8; ciphertext.len()];
        let data_bytes = Aes256CbcDec::new_from_slices(&key, &[0u8; 16]).unwrap()
            .decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buf).unwrap();

        let data_json = String::from_utf8(data_bytes.to_vec())?;
        let services = Services::from_json(&data_json)?;

        Ok(SecureServices {
            services,
            key,
        })
    }

    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        let data_json = self.services.to_json()?;
        let data_bytes = data_json.as_bytes();

        let buf_len = data_bytes.len() + 32;
        let mut buf = vec![0u8; buf_len];

        let ciphertext = Aes256CbcEnc::new_from_slices(&self.key, &[0u8; 16]).unwrap()
            .encrypt_padded_b2b_mut::<Pkcs7>(&data_bytes, &mut buf).unwrap();

        let mut file = File::create(VAULT_PATH)?;
        file.write_all(ciphertext)?;
        
        Ok(())
    }

    fn gen_key(password: String) -> [u8; 32] {
        let password_bytes = password.trim().as_bytes();
        let mut key = [0u8; 32];
        key[..password_bytes.len()].copy_from_slice(&password_bytes);
        key
    }

    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut Services {
        &mut self.services
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_and_load() {
        let _ = std::fs::remove_file("vault.json");

        let res = SecureServices::load("azerty123".into());
        assert!(res.is_err());

        let _ = SecureServices::create("azerty123".into());

        let secure_services = SecureServices::load("azerty123".into()).unwrap();
        assert_eq!(secure_services.services, Services::new(vec![]));
    }
}