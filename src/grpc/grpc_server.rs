use std::result;

use tonic::{transport::Server, Request, Response, Status, Streaming};

use upload_service::upload_service_server::{UploadService, UploadServiceServer};
use upload_service::{
    upload_request::Data, FileInfo, UploadRequest, UploadStatus, UploadStatusCode,
};

pub mod upload_service {
    tonic::include_proto!("upload");
}

use crate::utils;

#[derive(Debug, Default)]
pub struct Upload {}

#[tonic::async_trait]
impl UploadService for Upload {
    async fn upload(
        &self,
        request: Request<Streaming<UploadRequest>>,
    ) -> Result<Response<UploadStatus>, Status> {
        println!("Got a request: {:?}", request);

        let result = read_upload_request(request).await;

        if let Err(error) = result {
            return Ok(Response::new(UploadStatus {
                message: format!("{:?}", error),
                code: UploadStatusCode::Failed.into(),
            }));
        }

        let (file_data, chunks) = result.unwrap();

        println!("final stream: {:?}", &chunks);

        utils::hash::print(&chunks).expect("Error printing hash");

        let upload_status = match utils::file::write("data_server/output.txt", chunks) {
            Ok(()) => UploadStatus {
                message: format!("corretto"),
                code: UploadStatusCode::Ok.into(),
            },
            Err(error) => UploadStatus {
                message: format!("{:?}", error),
                code: UploadStatusCode::Failed.into(),
            },
        };

        Ok(Response::new(upload_status))
    }
}

async fn read_upload_request(
    request: Request<Streaming<UploadRequest>>,
) -> Result<(Option<FileInfo>, Vec<u8>), Box<dyn std::error::Error>> {
    let mut stream = request.into_inner();

    let mut file_info: Option<FileInfo> = None;
    let mut chunks = Vec::new();

    while let Some(upload_request) = stream.message().await? {
        match upload_request.data {
            Some(Data::FileInfo(info)) => {
                file_info = Some(info);
            }
            Some(Data::ChunkData(mut chunk_data)) => {
                chunks.append(&mut chunk_data);
            }
            None => {}
        }
    }
    Ok((file_info, chunks))
}

pub async fn server_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let upload = Upload::default();

    Server::builder()
        .add_service(UploadServiceServer::new(upload))
        .serve(addr)
        .await?;

    Ok(())
}
