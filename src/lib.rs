use serde::{Serialize, Deserialize};
use serde_json;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::error::Error;

mod service;
use service::Service;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Services {
    list: Vec<Service>,
}

impl Services {
    fn new(list: Vec<Service>) -> Services {
        Services { list }
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    pub fn add_service(
        &mut self,
        name: String, 
        email: String, 
        username: Option<String>
    ) -> Result<(), &'static str> {
        for s in &self.list {
            if &name == s.name() {
                return Err("Service with same name already exists")
            }
        }
        self.list.push(Service::new(name, email, username));
        Ok(())
    }

    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("vault.json")?;
        file.write_all(self.to_json()?.as_bytes())?;
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = Path::new("vault.json");

        if !path.exists() {
            let mut file = File::create("vault.json")?;
            let default_content = Services::new(vec![]).to_json()?;
            file.write_all(default_content.as_bytes())?;
        }
        
        let json_string = std::fs::read_to_string(&path)?;
        let services = Services::from_json(&json_string)?;
        Ok(services)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_store_and_load() {
        let s1 = Service::new("Gmail".into(), "toto@gmail.com".into(), None);
        let services = Services::new(vec![s1]);
        let _ = services.store();

        assert_eq!(services, Services::load().unwrap());
    }

    #[test]
    fn test_add_service() {
        let _ = std::fs::remove_file("vault.json");

        let mut services = Services::load().unwrap();
        let _ = services.add_service("Gmail".into(), "toto@gmail.com".into(), None);

        assert!(services.list.iter().any(|service| service.name() == "Gmail"));
    }

    #[test]
    fn test_add_same_service() {
        let _ = std::fs::remove_file("vault.json");

        let mut services = Services::load().unwrap();
        let _ = services.add_service("Gmail".into(), "toto@gmail.com".into(), None);
        let res = services.add_service("Gmail".into(), "tata@gmail.com".into(), None);

        assert!(res.is_err());
    }
}