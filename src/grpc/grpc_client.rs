use upload_service::upload_service_client::UploadServiceClient;
use upload_service::{upload_request::Data, FileInfo, UploadRequest};

pub mod upload_service {
    tonic::include_proto!("upload");
}

use crate::utils;

pub async fn upload_request(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UploadServiceClient::connect("http://[::1]:50051").await?;

    utils::hash::print(&data)?;

    let request_stream = async_stream::stream! {
        let file_info: FileInfo = FileInfo {
            file_type: "hello".to_string(),
            pc_id: "hello".to_string(),
        };

        let file_data: Option<Data> = Some(Data::FileInfo(file_info));
        yield UploadRequest {data: file_data};


        // let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        for byte in data.iter() {
            let bytes = vec![*byte];
            let chunk_data: Option<Data> = Some(Data::ChunkData(bytes));

            // interval.tick().await;

            println!("{:?}", utils::time::get_epoch_ms());
            yield UploadRequest {data: chunk_data};
        }
    };

    let response = client.upload(tonic::Request::new(request_stream)).await?;

    let inbound = response.into_inner();

    println!("upload status = {:?}", inbound.message);

    Ok(())
}
