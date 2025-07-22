use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 0..2 {
        let handle = tokio::spawn(async move {
            a(i).await;
        });
        handles.push(handle);
    }

    for i in handles {
        i.await.unwrap();
    }
}

async fn a(i: i32) {
    let v = call_db().await;
    println!("[{i}] Db call 1 {}", v);

    let v2 = call_db().await;
    println!("[{i}] Db call 2 {}", v2);
}

async fn call_db() -> String {
    tokio::time::sleep(Duration::from_millis(1000)).await;
    "database value".to_owned()
}
