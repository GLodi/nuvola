use rustgrpc::grpc;

extern crate diesel;
extern crate dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    grpc::grpc_server::server_main().await?;
    Ok(())
}
