use tonic::Request;

use upload_service::upload_service_client::UploadServiceClient;
use upload_service::Chunk;

pub mod upload_service {
    tonic::include_proto!("upload");
}

use crate::utils;

pub async fn upload_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UploadServiceClient::connect("http://[::1]:50051").await?;
    let file = utils::file::read("input.txt")?;

    utils::hash::print("input.txt")?;

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
