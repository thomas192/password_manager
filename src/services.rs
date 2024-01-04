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

    pub fn add(
        &mut self,
        name: String, 
        email: String, 
        username: Option<String>
    ) -> Result<(), &'static str> {
        for s in &self.list {
            if &name == s.name() {
                return Err("service with same name already exists");
            }
        }
        self.list.push(Service::new(name, email, username));
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), &'static str> {
        let length = self.list.len();
        if length == 0 {
            return Err("services list is empty")
        }
        self.list.retain(|s| name != s.name());
        if self.list.len() == length {
            return Err("unknown service name");
        }
        Ok(())
    }

    pub fn search(&self, name: &str) -> Result<Vec<&Service>, &'static str> {
        let matches: Vec<&Service> = self.list
            .iter()
            .filter(|s| s.name().contains(name))
            .collect();
        if matches.len() == 0 {
            return Err("unknown service name");
        }
        Ok(matches)
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
        let s1 = Service::new("gmail".into(), "toto@gmail.com".into(), None);
        let services = Services::new(vec![s1]);
        let _ = services.store();

        assert_eq!(services, Services::load().unwrap());
    }

    #[test]
    fn test_add() {
        let _ = std::fs::remove_file("vault.json");

        let mut services = Services::load().unwrap();
        let _ = services.add("gmail".into(), "toto@gmail.com".into(), None);
        assert!(services.list.iter().any(|service| service.name() == "gmail"));

        let res = services.add("gmail".into(), "tata@gmail.com".into(), None);
        assert!(res.is_err());
    }

    #[test]
    fn test_remove() {
        let _ = std::fs::remove_file("vault.json");

        let mut services = Services::load().unwrap();
        let _ = services.add("gmail".into(), "toto@gmail.com".into(), None);
        assert_eq!(1, services.list.len());

        let _ = services.remove("gmail");
        assert_eq!(0, services.list.len());

        let res = services.remove("gmail");
        assert!(res.is_err());
    }

    #[test]
    fn test_search() {
        let _ = std::fs::remove_file("vault.json");

        let mut services = Services::load().unwrap();
        let _ = services.add("gmail toto".into(), "toto@gmail.com".into(), None);
        let _ = services.add("gmail tata".into(), "tata@gmail.com".into(), None);
        let _ = services.add("mail titi".into(), "titi@mail.com".into(), None);

        let res = services.search("gmail").unwrap();
        assert_eq!(2, res.len())
    }
}