use tonic::{transport::Server, Request, Response, Status, Streaming};

use upload_service::upload_service_server::{UploadService, UploadServiceServer};
use upload_service::{Chunk, UploadStatus, UploadStatusCode};

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
        request: Request<Streaming<Chunk>>,
    ) -> Result<Response<UploadStatus>, Status> {
        println!("Got a request: {:?}", request);

        let mut stream = request.into_inner();

        let mut data: Vec<u8> = Vec::new();

        while let Some(mut chunk) = stream.message().await? {
            // println!("chunk: {:?}", &chunk.content);
            data.append(&mut chunk.content);
        }

        println!("final stream: {:?}", &data);

        utils::hash::print(&data).expect("Error printing hash");

        let upload_status = match utils::file::write("data_server/output.txt", data) {
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

pub async fn server_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let upload = Upload::default();

    Server::builder()
        .add_service(UploadServiceServer::new(upload))
        .serve(addr)
        .await?;

    Ok(())
}
