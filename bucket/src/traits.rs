use std::{fmt::{Debug, Display}, iter::Sum};

pub struct News {
    pub author: String,
    pub headline: String,
    pub content: String
}

pub struct Tweet {
    pub user: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }

    fn somethin(&self) -> String;
}


impl Summary for News  {
    // fn summarize(&self) -> String {
    //     format!("{} written by {}", self.headline, self.author)
    // }
    
    fn somethin(&self) -> String {
        String::from("THis news")
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} tweeted by {}", self.content, self.user)
    }

    fn somethin(&self) -> String {
        String::from("THis tweet")
    }
    
}

fn notify<T: Summary > (content: &T) {
    println!("Notified about this summary: {}", content.summarize());
}
// fn notify<T, U>(content: &T) where T: Summary + Display, U: Debug { // where clause after return type to add more readability to code
//     println!("Notified about this summary: {}", content.summarize());
// } // This example is incomplete, just to show that where clause can be added if there were multiple data types   

pub fn trat() {
    let n = News {
        author: String::from("NewsGuy"),
        headline: String::from("Good news"),
        content: String::from("There is some good news")
    };

    let t = Tweet {
        user: String::from("Tweeter"),
        content: String::from("this is a tweet body"),
        retweet: false,
        reply: false
    };

    let ns = n.summarize();
    let ts = t.summarize();

    println!("{}", ns);
    println!("{}", ts);

    notify(&n);
    notify(&t);
}