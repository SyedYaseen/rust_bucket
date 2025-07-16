use std::env;
use std::process;
use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // let config = match parse_args(&args) {
    //     Ok(c) => c,
    //     Err(e) => {
    //         println!("{}", e);
    //         process::exit(1)
    //     }
    // };

    let config = minigrep::parse_args(&args).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });

    
    if let Err(e) = minigrep::run(&config) {
        println!("App err: {}", e);
        process::exit(1);
    }

    process::exit(0)
}