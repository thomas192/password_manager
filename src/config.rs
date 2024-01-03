#[derive(Debug)]
pub enum Config {
    Add { name: String, email: String, username: Option<String> },
    Search { name: String },
    Remove { name: String },
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments: specify command");
        }
        let command = args[1].as_str();
        match command {
            "add" => {
                if args.len() < 4 {
                    return Err("not enough arguments for command add: \
specify name, email and username if any");
                }
                Ok(Config::Add {
                    name: args[2].clone(),
                    email: args[3].clone(),
                    username: args.get(4).cloned(),
                })
            },
            "search" => {
                if args.len() < 3 {
                    return Err("not enough arguments for command search: \
specify name");
                }
                Ok(Config::Search {
                    name: args[2].clone(),
                })
            },
            "remove" => {
                if args.len() < 3 {
                    return Err("not enough arguments for command remove: \
specify name");
                }
                Ok(Config::Remove {
                    name: args[2].clone(),
                })
            },
            _ => Err("unknown command"),
        }
    }
}