use serde::{Serialize, Deserialize};
use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    name: String,
    email: String,
    username: Option<String>,
    password: String,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Service: {}\nemail: {}\nusername: {}\npassword: {}", 
            self.name, self.email, self.username.as_ref().unwrap_or(&"None".to_string()), self.password
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
            password: Service::generate_password(),
        }
    }

    fn generate_password() -> String {
        let ascii_characters = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789!@#$%^&*()";
        let mut rng = rand::thread_rng();
        let password: String = (0..20)
                                .map(|_| {
                                    let index = rng.gen_range(0..ascii_characters.len());
                                    ascii_characters[index] as char
                                })
                                .collect();
        password
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_ref().map(|s| s.as_str())
    }

    pub fn password(&self) -> &str {
        &self.password
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