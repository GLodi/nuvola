use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

extern crate reed_solomon_erasure;

use rustgrpc::grpc;
use rustgrpc::utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let (tx, rx) = channel();

    // let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    // watcher
    //     .watch(
    //         "/Users/giulio/dev/rustgrpc/data_client",
    //         RecursiveMode::Recursive,
    //     )
    //     .unwrap();

    // loop {
    //     match rx.recv() {
    //         Ok(event) => println!("{:?}", event),
    //         Err(e) => println!("watch error: {:?}", e),
    //     }
    // }

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
