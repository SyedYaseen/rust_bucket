use std::{
    os::unix::thread,
    sync::{Arc, Mutex, mpsc},
    thread::{JoinHandle, sleep, spawn},
    time::Duration,
};

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
    let m1 = vec![
        String::from("m1"),
        String::from("m2"),
        String::from("m3"),
        String::from("m4"),
    ];

    let m2 = vec![
        String::from("m11"),
        String::from("m22"),
        String::from("m33"),
        String::from("m44"),
        String::from("m55"),
    ];

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

fn mutx() {
    // Basics
    let m = Mutex::new(5);
    let mut num: std::sync::MutexGuard<'_, i32> = m.lock().unwrap();
    *num = 6;
    println!("{:?}", m);
}

fn mutx_arc() {
    // Spawn 10 threads that inc mutex value by 1
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let m_arc = Arc::clone(&m);
        let handle = spawn(move || {
            let mut g = m_arc.lock().unwrap();
            *g += 1;
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{:#?}", m);
}

fn main() {
    // basics();
    // messages();
    // mutx();
    mutx_arc();
}
