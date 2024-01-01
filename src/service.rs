use rand::Rng;

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
    pub password: String,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        assert_eq!(20, Service::generate_password().len());
    }
}