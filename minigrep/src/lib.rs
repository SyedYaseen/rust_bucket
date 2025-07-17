use std::fs;
use std::error::Error;
use std::env;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String
}

impl Config {
    fn new(q: String, f: String) -> Config {
        Config {
            query: q,
            file_name: f
        }
    }
}

pub fn parse_args<T>(args: &mut T) -> Result<Config, &str> where T: Iterator<Item = String> {
    let _ = args.next();

    let q = match args.next() {
        Some(q) => q,
        None => {
            println!("Please enter a query");
            process::exit(1);
        }
    };

    let f = match args.next() {
        Some(f) => f,
        None => {
            println!("Please enter a filename");
            process::exit(1);
        }
    };

    Ok(Config::new(q, f))
}

pub fn run(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_name)?;
    
    let found_str= search(&contents, &config.query);
    Ok(found_str)
}

pub fn search(contents: &String, query: &String) -> Vec<String> {
    contents
    .lines()
    .filter(|l| l.contains(query))
    .map(|l| l.to_string())
    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_on_valid_args() {
        let args_vec = vec![
            "binary_name".to_string(),
            "search_term".to_string(),
            "filename.txt".to_string(),
        ];

        let mut args_iter = args_vec.into_iter();
        let config = parse_args(&mut args_iter).expect("Should parse config");

        assert_eq!(config.query, "search_term");
        assert_eq!(config.file_name, "filename.txt");
        
    }

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
        let str_list = search(&contents, &query);
        assert_eq!(str_list, vec!["how to bake potatoes", "baked briyani"]);
    }
}

