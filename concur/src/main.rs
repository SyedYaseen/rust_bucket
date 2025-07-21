use std::{os::unix::thread, sync::mpsc, thread::{sleep, spawn}, time::Duration};

fn basics() {
    let th = spawn(|| {
        for i in 1..10 {
            println!("Thread: {i}");
            sleep(Duration::from_millis(1));
        }
    });
    // th.join();

    for i in 1..5 {
        println!("Main: {i}");
        sleep(Duration::from_millis(1));
    }
    th.join();
}

fn messages() {
    
    let m1 = vec! {
        String::from("m1"),
        String::from("m2"),
        String::from("m3"),
        String::from("m4"),
    };

    let m2 = vec! {
        String::from("m11"),
        String::from("m22"),
        String::from("m33"),
        String::from("m44"),
        String::from("m55"),
    };

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // Must be cloned here else it will be consumed.

    let h1 = spawn(move || {
        for i in m1 {
            tx.send(i).unwrap(); // Handle gracecfully without unwrap
            sleep(Duration::from_millis(1));
        }
    });

    
    let h2 = spawn(move || {
        for i in m2 {
            tx2.send(i).unwrap();
            sleep(Duration::from_millis(1));
        }
    });

    let h3 = spawn(move || {
        for i in rx {
            println!("{i}");
        }
    });

    h3.join();

    // for i in rx {
    //     println!("{i}");
    // }


}

fn main() {
    // basics();
    messages();


}
