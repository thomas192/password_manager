use serde::{Serialize, Deserialize};
use serde_json;

mod service;
use service::Service;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Services {
    list: Vec<Service>,
}

impl Services {
    pub fn new(list: Vec<Service>) -> Services {
        Services { list }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut services = Services::new(vec![]);
        let _ = services.add("gmail".into(), "toto@gmail.com".into(), None);
        assert!(services.list.iter().any(|service| service.name() == "gmail"));

        let res = services.add("gmail".into(), "tata@gmail.com".into(), None);
        assert!(res.is_err());
    }

    #[test]
    fn test_remove() {
        let mut services = Services::new(vec![]);
        let _ = services.add("gmail".into(), "toto@gmail.com".into(), None);
        assert_eq!(1, services.list.len());

        let _ = services.remove("gmail");
        assert_eq!(0, services.list.len());

        let res = services.remove("gmail");
        assert!(res.is_err());
    }

    #[test]
    fn test_search() {
        let mut services = Services::new(vec![]);
        let _ = services.add("gmail toto".into(), "toto@gmail.com".into(), None);
        let _ = services.add("gmail tata".into(), "tata@gmail.com".into(), None);
        let _ = services.add("mail titi".into(), "titi@mail.com".into(), None);

        let res = services.search("gmail").unwrap();
        assert_eq!(2, res.len())
    }
}