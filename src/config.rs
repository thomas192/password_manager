#[derive(Debug)]
pub enum Config {
    Create,
    Add { name: String, email: String, username: Option<String> },
    Search { name: String },
    Remove { name: String },
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments, specify command");
        }
        let command = args[1].as_str();
        match command {
            "create" => {
                Ok(Config::Create)
            },
            "add" => {
                if args.len() < 4 {
                    return Err("not enough arguments for command add");
                }
                Ok(Config::Add {
                    name: args[2].clone(),
                    email: args[3].clone(),
                    username: args.get(4).cloned(),
                })
            },
            "search" => {
                if args.len() < 3 {
                    return Err("not enough arguments for command search");
                }
                Ok(Config::Search {
                    name: args[2].clone(),
                })
            },
            "remove" => {
                if args.len() < 3 {
                    return Err("not enough arguments for command remove");
                }
                Ok(Config::Remove {
                    name: args[2].clone(),
                })
            },
            _ => Err("unknown command"),
        }
    }
}