use std::{fmt::Error, fs::{self, File}, io::{self, ErrorKind, Read}};

pub fn err_handl() {
    // panic!("This is err out");
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(f) => f,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create_new("hello.txt") {
    //             Ok(f) => f,
    //             Err(err) => panic!("Err creating file {}", err)
    //         },
    //         other => panic!("Err kind: {}, Full err: {}", other, err)
    //     }
    // };

    // let f2 = File::open("hello2.txt").expect("Er opening file");
    // let f2 = File::open("hello2.txt").unwrap();
    let a = read_from_file();
    match a {
        Ok(a) => println!("{}", a),
        Err(e) => println!("{}", e)
    }

}

fn read_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt").unwrap();
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(b) => println!("read {} bytes", b),
    //     Err(e) => return Err(e)
    // }
    // Ok(s)

    // Simplifying above
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // Simplifying above
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // Simplify above
    fs::read_to_string("hello.txt")
    

}