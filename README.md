# Password Manager

A minimalistic command-line password manager I developed halfway through "The Rust Programming Language" book, to get familiar with Rust.

Security Features :
- Key Derivation: Leverages PBKDF2 with SHA256 hashing to securely generate encryption keys from passwords, enhancing the strength against brute-force attacks.
- Data Encryption and Storage: Utilizes AES-256 encryption combined with PKCS7 padding to securely encrypt data before storing it in a file-based vault.
- Memory Safety: Implements zeroization to securely erase sensitive information from memory once it's no longer needed, preventing potential data leaks.
