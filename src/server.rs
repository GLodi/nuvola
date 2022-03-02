use tonic::{transport::Server, Request, Response, Status, Streaming};

use upload_service::upload_service_server::{UploadService, UploadServiceServer};
use upload_service::{Chunk, UploadStatus, UploadStatusCode};

pub mod upload_service {
    tonic::include_proto!("upload");
}

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
            println!("chunk: {:?}", std::str::from_utf8(&chunk.content).unwrap());
            data.append(&mut chunk.content);
        }

        println!("final stream: {:?}", &data);

        match write_to_file(data) {
            Ok(ok) => {}
            Err(error) => {}
        };

        let upload_status = UploadStatus {
            message: format!("ahah"),
            code: UploadStatusCode::Ok.into(),
        };

        Ok(Response::new(upload_status))
    }
}

async fn write_to_file(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let upload = Upload::default();

    Server::builder()
        .add_service(UploadServiceServer::new(upload))
        .serve(addr)
        .await?;

    Ok(())
}
