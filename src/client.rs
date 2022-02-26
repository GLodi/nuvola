use tonic::Request;

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use upload_service::upload_service_client::UploadServiceClient;
use upload_service::{Chunk, UploadStatus, UploadStatusCode};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod upload_service {
    tonic::include_proto!("upload");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    upload_request().await?;
    Ok(())
}

async fn hello_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn upload_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UploadServiceClient::connect("http://[::1]:50051").await?;

    let outbound = async_stream::stream! {
        for _ in 1..5 {
            let chunk = Chunk {
                content: "1234".as_bytes().to_vec(),
            };

            yield chunk;
        }
    };

    let response = client.upload(Request::new(outbound)).await?;
    let inbound = response.into_inner();

    println!("NOTE = {:?}", inbound.message);

    Ok(())
}
