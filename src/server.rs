use tonic::{Request, Response, Status, transport::Server};

use gateway::gateway_server::{Gateway, GatewayServer};
use gateway::{Null, VersionResponse};

// Import the generated proto-rust file into a module
pub mod gateway {
    tonic::include_proto!("com.example.gateway");
}

// Implement the service skeleton for the "Greeter" service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyGreeter {}

// Implement the service function(s) defined in the proto
// for the Greeter service (SayHello...)
#[tonic::async_trait]
impl Gateway for MyGreeter {
    async fn grpc_version(
        &self,
        request: Request<Null>,
    ) -> Result<Response<VersionResponse>, Status> {
        println!("Received request from: {:?}", request);

        let response = gateway::VersionResponse {
            base_response: Some(gateway::BaseResponse {
                status: 200,
                message: "OK".to_string(),
            }),
            data: "1.0.0".to_string(),
        };

        Ok(Response::new(response))
    }
}

// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let gateway = MyGreeter::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(GatewayServer::new(gateway))
        .serve(addr)
        .await?;

    Ok(())
}
