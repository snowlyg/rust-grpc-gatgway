use gateway::gateway_client::GatewayClient;

pub mod gateway {
    tonic::include_proto!("com.example.gateway");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GatewayClient::connect("http://localhost:7880").await?;
    let request = tonic::Request::new(gateway::Null {});
    // println!("Sending request to gRPC Server...");
    let response = client.grpc_version(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
