use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{watcher, RecursiveMode, Watcher};

use nuvola::db;
use nuvola::enc;
use nuvola::fs;
use nuvola::grpc;

extern crate dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // send_file().await;
    // encrypt();
    db::init::main();
    Ok(())
}

fn encrypt() {
    let file_vec = match fs::file::read("data_client/input.txt") {
        Ok(buffer) => buffer,
        Err(_) => panic!("no file at location"),
    };
    enc::rsa::encrypt_file(&file_vec);
}

async fn observe() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher
        .watch(
            "/Users/giulio/dev/nuvola/data_client",
            RecursiveMode::Recursive,
        )
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

async fn send_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_vec = match fs::file::read("data_client/input.txt") {
        Ok(buffer) => buffer,
        Err(_) => panic!("no file at location"),
    };

    let encoded_file = fs::reed::encode(&file_vec);

    println!("encoded stream sending:");
    println!("{encoded_file:?}");

    grpc::client::upload_request(encoded_file).await?;

    Ok(())
}
