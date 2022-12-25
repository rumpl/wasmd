use std::sync::Arc;

use log::{debug, info};
use tonic::{transport::Server, Request, Response, Status};
use wasmd_proto::runtime::daemon_server::{Daemon, DaemonServer};
use wasmd_proto::runtime::{RunRequest, RunResponse};

#[derive(Default)]
pub struct WasmdServer {}

impl WasmdServer {
    pub fn stop(&self) {
        info!("stopping wasmd");
    }
}

#[tonic::async_trait]
impl Daemon for WasmdServer {
    async fn run(&self, request: Request<RunRequest>) -> Result<Response<RunResponse>, Status> {
        let req = request.get_ref();
        debug!("run request {:?}", req);

        Ok(Response::new(RunResponse {
            image: req.image.clone(),
            message: String::from("Hello from wasmd"),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "[::1]:50052".parse()?;
    let server = Arc::new(WasmdServer::default());

    info!("wasmd listening on {}", addr);

    Ok(Server::builder()
        .add_service(DaemonServer::from_arc(server.clone()))
        .serve_with_shutdown(addr, async {
            tokio::signal::ctrl_c().await.unwrap();
            server.stop();
        })
        .await?)
}
