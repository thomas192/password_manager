use aes::cipher::block_padding::UnpadError;
use serde_json;

#[derive(Debug)]
pub enum SecureServiceError {
    IoError(std::io::Error),
    VaultFileNotFound,
    VaultFileCorrupted,
    CryptoError,
}

impl std::error::Error for SecureServiceError {}

impl From<std::io::Error> for SecureServiceError {
    fn from(err: std::io::Error) -> Self {
        SecureServiceError::IoError(err)
    }
}

impl From<std::string::FromUtf8Error> for SecureServiceError {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        SecureServiceError::VaultFileCorrupted
    }
}

impl From<serde_json::Error> for SecureServiceError {
    fn from(_err: serde_json::Error) -> Self {
        SecureServiceError::VaultFileCorrupted
    }
}

impl From<UnpadError> for SecureServiceError {
    fn from(_err: UnpadError) -> Self {
        SecureServiceError::CryptoError
    }
}

impl std::fmt::Display for SecureServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            SecureServiceError::IoError(ref err) => write!(f, "could not read/write vault file\n{}", err),
            SecureServiceError::VaultFileNotFound => write!(f, "vault file not found"),
            SecureServiceError::VaultFileCorrupted => write!(f, "vault file corrupted"),
            SecureServiceError::CryptoError => write!(f, "invalid password or corrupted vault file"),
        }
    }
}