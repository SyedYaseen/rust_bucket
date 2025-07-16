use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String
}

impl Config {
    fn new(q: &String, f: &String) -> Config {
        Config {
            query: q.clone(),
            file_name: f.clone()
        }
    }
}

pub fn parse_args(args: &Vec<String>) -> Result<Config, &str> {
    if args.len() < 3 {
        return Err("Not enough args") // Since there is no else block we need to use "return" keyword. Else just the returned obj would be enough
    }

    Ok(Config::new(&args[1], &args[2]))
}

pub fn run(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_name)?;
    let mut found_str: Vec<String> = Vec::new();
    search(&contents, &config.query, &mut found_str);

    Ok(found_str)
}

pub fn search<'a>(contents: &'a String, query: &String, found_str: &mut Vec<String>) {
    
    for line in contents.lines() {
        if line.contains(query) {
            found_str.push(line.to_string());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_text() {
        let contents = String::from("\
This is a sample
frog man
how to bake potatoes
rubber ducky is my friend
baked briyani
        ");
        let query = String::from("bake");
        
        let out = search(&contents, &query);
        assert_eq!(out, vec!["how to bake potatoes", "baked briyani"]);
    }
}

