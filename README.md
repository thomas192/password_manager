# Password Manager

A minimalistic command-line password manager I developed halfway through "The Rust Programming Language" book, to get familiar with Rust.

Security Features :
- Key Derivation: Leverages PBKDF2 with SHA256 hashing to securely generate encryption keys from passwords, enhancing the strength against brute-force attacks.
- Data Encryption and Storage: Utilizes AES-256 encryption combined with PKCS7 padding to securely encrypt data before storing it in a file-based vault.
- Memory Safety: Implements zeroization to securely erase sensitive information from memory once it's no longer needed, preventing potential data leaks.

Usage :
- pwdm create
- pwdm add "tom's mail" "tom@mail"
- pwdm add "tom's 2nd mail" "tom2@mail.com" "tom123"
- pwdm search "mail"
Output :
```
Service: tom's mail
email: tom@mail.com
username: None
password: O&foVssBbblQbq%RLags

Service: tom's 2nd mail
email: tom2@mail.com
username: tom123
password: )SA422C6h#FIES3Ok7nu
```
- pwdm remove "tom's mail"
