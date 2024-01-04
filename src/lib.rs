use std::error::Error;

mod services;
use services::Services;
pub mod config;
use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Add { name, email, username } => {
            let mut services = Services::load()?;
            services.add(name, email, username)?;
            services.store()?;
            println!("service added successfully");
            Ok(())
        },
        Config::Search { name } => {
            let services = Services::load()?;
            let matches = services.search(&name)?;
            for s in matches {
                println!("{}\n", s);
            }
            Ok(())
        },
        Config::Remove { name } => {
            let mut services = Services::load()?;
            services.remove(&name)?;
            services.store()?;
            println!("service removed successfully");
            Ok(())
        },
    }
}
