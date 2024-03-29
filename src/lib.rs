use std::error::Error;
use rpassword;
use zeroize::Zeroizing;

mod secure_services;
use secure_services::SecureServices;
pub mod config;
use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let password = Zeroizing::new(rpassword::prompt_password("Enter password: ")?);

    match config {
        Config::Create => {
            SecureServices::create(password)?;
            println!("vault created successfully");
            Ok(())
        },
        Config::Add { name, email, username } => {
            let mut ss = SecureServices::load(password)?;
            ss.services_mut().add(name, email, username)?;
            ss.store()?;
            println!("service added successfully");
            Ok(())
        },
        Config::Search { name } => {
            let ss = SecureServices::load(password)?;
            let matches = ss.services().search(&name)?;
            for s in matches {
                println!("{}\n", s);
            }
            Ok(())
        },
        Config::Remove { name } => {
            let mut ss = SecureServices::load(password)?;
            ss.services_mut().remove(&name)?;
            ss.store()?;
            println!("service removed successfully");
            Ok(())
        },
    }
}
