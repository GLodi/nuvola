use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

extern crate reed_solomon_erasure;

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

    // grpc::grpc_client::upload_request().await?;

    let file_vec = match utils::file::read("data_client/input.txt") {
        Ok(buffer) => buffer,
        Err(e) => panic!("no file at location"),
    };

    utils::reed::encode(file_vec);
    Ok(())
}
