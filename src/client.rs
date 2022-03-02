use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use sha2::{Digest, Sha256};
use tonic::Request;

use upload_service::upload_service_client::UploadServiceClient;
use upload_service::Chunk;

pub mod upload_service {
    tonic::include_proto!("upload");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    upload_request().await?;
    Ok(())
}

async fn upload_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UploadServiceClient::connect("http://[::1]:50051").await?;
    let file = read_file().await?;

    let outbound = async_stream::stream! {
        for byte in file.iter() {
            let chunk = Chunk {
                content: vec![*byte],
            };

            yield chunk;
        }
    };

    let response = client.upload(Request::new(outbound)).await?;
    let inbound = response.into_inner();

    println!("upload status = {:?}", inbound.message);

    Ok(())
}

async fn read_file() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    print_hash()?;

    // Read.
    for value in &buffer {
        println!("BYTE: {}", value);
    }

    Ok(buffer)
}

fn print_hash() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut hasher = sha2::Sha256::new();
    let n = std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    println!("file hash: {:?}", hash);
    Ok(())
}
