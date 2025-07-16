pub fn ownership() {
    // let aref = dangling();
    let s = String::from("Hello world");
    let fw = first_word(&s);
    println!("{}", fw);
}
fn first_word(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

// pub fn dangling() -> &String {
//     let a = String:from("Hello");

//     &a;
// }
