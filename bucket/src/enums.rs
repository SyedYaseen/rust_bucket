enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Faces {
    Lincoln,
    Bush,
    Obama,
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(Faces),
}

pub fn enums() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    let x = 5;
    let y: Option<i32> = None;

    let z = x + y.unwrap_or(0);
    println!("{}", z);

    let coin = Coins::Quarter(Faces::Lincoln);

    // let val = coin_values(Coins::Quarter(Faces::Obama));
    let val = coin_values(&coin);
    println!("Coin val is: {}", val);

    match_onecase();
}

fn coin_values(coin: &Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 10,
        Coins::Dime => 50,
        Coins::Quarter(face) => {
            println!("{:?} is on the coin", face);
            25
        }
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }                     
}

fn match_onecase() {
    let x = Some(3);
    match x {
        Some(3) => println!("Matches exactly"),
        _ => (),
    }

    // Below is same as above
    if let Some(3) = x {
        println!("Matches exactly")
    }
}
