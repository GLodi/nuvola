use tonic::Request;

use upload_service::upload_service_client::UploadServiceClient;
use upload_service::Chunk;

pub mod upload_service {
    tonic::include_proto!("upload");
}

use crate::utils;

pub async fn upload_request(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UploadServiceClient::connect("http://[::1]:50051").await?;

    utils::hash::print(&data)?;

    let outbound = async_stream::stream! {
        for byte in data.iter() {
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
