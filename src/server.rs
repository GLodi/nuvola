use nuvola::grpc;

extern crate dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    grpc::server::server_main().await?;
    Ok(())
}
