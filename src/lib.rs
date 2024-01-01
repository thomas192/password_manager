use rand::Rng;

pub fn generate_password() -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        assert_eq!(20, generate_password().len());
    }
}