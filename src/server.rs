use rustgrpc::grpc;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
use db::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    grpc::grpc_server::server_main().await?;
    Ok(())
}
