use upload_service::upload_service_server::{UploadService, UploadServiceServer};
use upload_service::{
    upload_request::Data, FileInfo, UploadRequest, UploadStatus, UploadStatusCode,
};

pub mod upload_service {
    tonic::include_proto!("upload");
}

use crate::fs;
use crate::utils;

#[derive(Debug, Default)]
pub struct Upload {}

#[tonic::async_trait]
impl UploadService for Upload {
    async fn upload(
        &self,
        request: tonic::Request<tonic::Streaming<UploadRequest>>,
    ) -> Result<tonic::Response<UploadStatus>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let result = read_upload_request(request).await;

        if let Err(error) = result {
            return Ok(tonic::Response::new(UploadStatus {
                message: format!("{:?}", error),
                code: UploadStatusCode::Failed.into(),
            }));
        }

        let (file_info, chunks) = result.unwrap();

        if let Some(file_info) = file_info {
            println!("final stream file_info: {:?}", file_info.file_type);
        }
        println!("final stream chunks: {:?}", &chunks);

        fs::hash::print(&chunks).expect("Error printing hash");

        let upload_status = match fs::file::write("data_server/output.txt", chunks) {
            Ok(()) => UploadStatus {
                message: format!("corretto"),
                code: UploadStatusCode::Ok.into(),
            },
            Err(error) => UploadStatus {
                message: format!("{:?}", error),
                code: UploadStatusCode::Failed.into(),
            },
        };

        Ok(tonic::Response::new(upload_status))
    }
}

async fn read_upload_request(
    request: tonic::Request<tonic::Streaming<UploadRequest>>,
) -> Result<(Option<FileInfo>, Vec<u8>), Box<dyn std::error::Error>> {
    let mut stream = request.into_inner();

    let mut file_info: Option<FileInfo> = None;
    let mut chunks = Vec::new();

    while let Some(upload_request) = stream.message().await? {
        println!("{:?}", utils::time::get_epoch_ms());

        match upload_request.data {
            Some(Data::FileInfo(info)) => {
                file_info = Some(info);
            }
            Some(Data::ChunkData(mut data_chunk)) => {
                chunks.append(&mut data_chunk);
            }
            None => {}
        }
    }

    println!("done");
    Ok((file_info, chunks))
}

pub async fn server_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let upload = Upload::default();

    tonic::transport::Server::builder()
        .timeout(std::time::Duration::from_secs(10))
        .add_service(UploadServiceServer::new(upload))
        .serve(addr)
        .await?;

    Ok(())
}
