#[derive(Debug)]
pub enum Config {
    Create,
    Add { name: String, email: String, username: Option<String> },
    Search { name: String },
    Remove { name: String },
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments, specify command"),
        };

        let command = command.as_str();
        
        if command == "create" {
            return Ok(Config::Create);
        }

        let name = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments, service name required"),
        };

        match command {
            "add" => {
                let email = match args.next() {
                    Some(arg) => arg,
                    None => return Err("not enough arguments for command add, email required")
                };
                Ok(Config::Add {
                    name: name,
                    email: email,
                    username: args.next(),
                })
            },
            "search" => {
                Ok(Config::Search {
                    name: name,
                })
            },
            "remove" => {
                Ok(Config::Remove {
                    name: name,
                })
            },
            _ => Err("unknown command"),
        }
    }
}