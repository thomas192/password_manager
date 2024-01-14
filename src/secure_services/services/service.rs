use serde::{Serialize, Deserialize};
use rand::Rng;
use std::fmt;
use zeroize::Zeroizing;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    name: String,
    email: String,
    username: Option<String>,
    password: Zeroizing<String>,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Service: {}\nemail: {}\nusername: {}\npassword: {}", 
            self.name, self.email, self.username.as_ref().unwrap_or(&"None".to_string()), *self.password
        )
    }
}

impl Service {
    pub fn new(
        name: String, 
        email: String,
        username: Option<String>
    ) -> Service {
        Service {
            name,
            email,
            username,
            password: Zeroizing::new(Service::generate_password()),
        }
    }

    fn generate_password() -> String {
        let ascii_characters = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789!@#$%^&*()";
        let mut rng = rand::thread_rng();
        let mut password = String::with_capacity(20); // pre-allocate for safety
        for _ in 0..20 {
            let index = rng.gen_range(0..ascii_characters.len());
            password.push(ascii_characters[index] as char)
        }
        password
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        assert_eq!(20, Service::generate_password().len());
    }
}