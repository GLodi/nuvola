use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{watcher, RecursiveMode, Watcher};

use rustgrpc::grpc;
use rustgrpc::utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate reed_solomon_erasure;

mod db;
use db::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    send_file().await;
    Ok(())
}

async fn observe() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher
        .watch(
            "/Users/giulio/dev/rustgrpc/data_client",
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
    let file_vec = match utils::file::read("data_client/input.txt") {
        Ok(buffer) => buffer,
        Err(_) => panic!("no file at location"),
    };

    let encoded_file = utils::reed::encode(&file_vec);

    println!("encoded stream sending:");
    println!("{encoded_file:?}");

    grpc::grpc_client::upload_request(encoded_file).await?;

    Ok(())
}
