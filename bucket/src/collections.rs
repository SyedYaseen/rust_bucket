use unicode_segmentation::{ UnicodeSegmentation};
use std::{self, collections::HashMap};
pub fn collections() {
    // vector_basics();
    // SpreadSheet();
    // StringsTut();
    HashMpas();
}

fn HashMpas() {
    let mut hm = HashMap::new();
    hm.insert(String::from("hello"), String::from("world"));
    hm.insert(String::from("foo"), String::from("bar"));
    let ke =String::from("hello");
    
    let v = hm.get(&ke);
    if let Some(i) = v {
        println!("Received {}", i);
    } else {
        println!("Key dont ezits");
    }

    // insert on same key will update value, it we dont want update if key present, use entry with .or_insert
    hm.entry(String::from("marco")).or_insert(String::from("polo"));
    hm.entry(String::from("marco")).or_insert(String::from("oolo")); // this wont be added
    
    

    for (k,v) in &hm {
        println!("{}: {}", k, v);
    }

    // sample 2
    let mut chr_count: HashMap<String, u32> = HashMap::new();
    let sample = String::from("aaa bb cccc");

    for i in sample.chars() {
        let key = if i == ' ' { "space".to_string() } else { i.to_string() }; // this is same as below

        // let mut key: String = i.to_string();
        // if (key == " ") {
        //     key = String::from("space");
        // }

        let counter = chr_count.entry(key).or_insert(0); // or_insert returns a mutable ref to the value
        *counter += 1;
    }

    for (k,v) in &chr_count {
        println!("{}: {}", k, v);
    }
}

fn StringsTut() {
    let a = String::from("hello");
    let b = &a;
    let c = &a[..];
    let mut d: &str = "world";

    let mut e = String::new();
    e = a + " " + d;
    println!("{}", e);
    
    for i in "helo".graphemes(true) {
        println!("{:?}", i);
    }

    let g = "wor".graphemes(true);
    println!("{:?}", g);
    

    let gr: String = String::from("somestr");
    let graph_str: unicode_segmentation::Graphemes<'_> = gr.graphemes(true);
    for i in graph_str {
        println!("{:?}", i);
    }

    println!("{:?}", gr);

    // for i in graph_str { // This isnt possible because .graphemes(true) creates an iterator and for in consumes that iterator
    //     println!("{:?}", i);
    // }

    for i in gr.graphemes(true) { // So we create a new iterator. Note that graphemes uses &self so it implicitly takes a ref of the string it uses. So the ownership doesnt change.
         println!("{:?}", i);
    }



    // let gr = a.graphemes(true);


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
