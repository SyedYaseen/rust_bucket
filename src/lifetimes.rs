use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// pub fn lifeti() {
//     let x = String::from("aaaa");
//     let y = String::from("aa");
//     longest(&x, &y);
// }

// fn lifeti() {
//     let string1 = String::from("long string is long");
//     let result;
//     let string2 = String::from("xyz");
//     {
//         result = longest(&string1, &string2);
//     }
//     println!("The longest string is '{result}'");
// }


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn wrap_up_section<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement is {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


pub fn lifeti() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    // println!("{} by {}", book.title, book.author);

    let x = String::from("aaa");
    let y = String::from("aa");
    let ann: String = String::from("The next station is");
    let out = wrap_up_section(&x, &y, ann);
    println!("op from wrap: {}", out);
}

