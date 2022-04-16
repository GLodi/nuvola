use tonic::Request;

use upload_service::upload_service_client::UploadServiceClient;
use upload_service::{upload_request::Data, ImageInfo, UploadRequest};

pub mod upload_service {
    tonic::include_proto!("upload");
}

use crate::utils;

pub async fn upload_request(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UploadServiceClient::connect("http://[::1]:50051").await?;

    utils::hash::print(&data)?;

    let request_stream = async_stream::stream! {
        let image_info: ImageInfo = ImageInfo {
            image_type: "hello".to_string(),
            pc_id: "hello".to_string(),
        };

        let image_data: Option<Data> = Some(Data::ImageInfo(image_info));
        yield UploadRequest {data: image_data};

        for byte in data.iter() {

            // let chunk_data = upload_service::upload_request:: {

            //     chunk_data: vec![*byte],
            // };

            // yield chunk_data;
            // yield UploadRequest {data: data};
        }
    };

    let response = client.upload(Request::new(request_stream)).await?;
    let inbound = response.into_inner();

    println!("upload status = {:?}", inbound.message);

    Ok(())
}
