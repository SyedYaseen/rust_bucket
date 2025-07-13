pub fn collections() {
    // vector_basics();
    // SpreadSheet();
    StringsTut();
}

fn StringsTut() {
    let a = String::from("hello");
    let b = &a;
    let c = &a[..];
}

enum SpreadsheetCell {
    Integer(i32),
    Text(String),
    Enabled(bool),
}

fn SpreadSheet() {
    let mut row = vec![
        SpreadsheetCell::Integer(5),
        SpreadsheetCell::Text(String::from("This is awesome")),
        SpreadsheetCell::Enabled(false),
    ];

    match &row[0] {
        SpreadsheetCell::Integer(i) => println!("This is an int"),
        _ => println!("Not int"),
    }
}

fn vector_basics() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4];
    v.push(1);
    println!("{}", &v[0]);

    let binding = String::from("Hello");
    let vStr = vec![binding.chars()];
    let s = &vStr[0]; // ❌ ERROR: move occurs here
    println!("{:?}", s);
    // println!("{}", vStr[0]);
    // println!("{}", vStr[0]);

    let mut vStr2: Vec<char> = Vec::new();
    for i in binding.chars() {
        vStr2.push(i);
    }
    println!("{:?}", vStr2[0]);

    // for i in &vStr {
    //     println!("{}", i);
    // }
}
