use tonic::{transport::Server, Request, Response, Status};

use twodipd::ipd_server::{Ipd, IpdServer};
use twodipd::join_response::JoinStatus;
use twodipd::{JoinRequest, JoinResponse};

pub mod twodipd {
    tonic::include_proto!("twodipd"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyIpd {}

#[tonic::async_trait]
impl Ipd for MyIpd {
    async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = twodipd::JoinResponse {
            status: JoinStatus::Success as i32,
            status_oneof: Some(twodipd::join_response::StatusOneof::Gameid(42)),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let join = MyIpd::default();

    Server::builder()
        .add_service(IpdServer::new(join))
        .serve(addr)
        .await?;

    Ok(())
}
