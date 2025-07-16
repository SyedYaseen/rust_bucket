use std::env;
use std::error::Error;
use std::process;
use std::fs;

#[derive(Debug)]
struct Config {
    query: String,
    file_name: String
}

impl Config {
    fn new(q: &String, f: &String) -> Config {
        Config {
            query: q.clone(),
            file_name: f.clone()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = match parse_args(&args) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    };
    

    // if let Error = fs::read_to_string(config.file_name)
    if let Err(e) = run(&config) {
        println!("App err: {}", e);
        process::exit(1);
    }

    
}

fn parse_args(args: &Vec<String>) -> Result<Config, &str> {
    if args.len() < 3 {
        return Err("Not enough args")
    }

    Ok(Config::new(&args[1], &args[2]))
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let s = fs::read_to_string(&config.file_name)?;
    println!("{:#?}", s);
    Ok(())

}
