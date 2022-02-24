use tonic::{transport::Server, Request, Response, Status, Streaming};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use upload_service::upload_service_server::{UploadService, UploadServiceServer};
use upload_service::{Chunk, UploadStatus, UploadStatusCode};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod upload_service {
    tonic::include_proto!("upload");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
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

        while let Some(chunk) = stream.message().await? {
            println!("chunk: {:?}", chunk);
        }

        let upload_status = UploadStatus {
            message: format!("ahah"),
            code: UploadStatusCode::Ok.into(),
        };

        Ok(Response::new(upload_status))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let upload = Upload::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(UploadServiceServer::new(upload))
        .serve(addr)
        .await?;

    Ok(())
}
