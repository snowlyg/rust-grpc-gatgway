use clap::{Parser, Subcommand};
use gateway::{ApplyRequest, CommonRequest, Null, RegisterRequest, gateway_client::GatewayClient};

pub mod gateway {
    tonic::include_proto!("_");
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version {
        url: Option<String>,
    },
    Register {
        url: Option<String>,
        to_ip: Option<String>,
        num: Option<String>,
        client_type: Option<String>,
        ip: Option<String>,
        identity: Option<String>,
        display_name: Option<String>,
        grpc_port: Option<i64>,
        http_port: Option<i64>,
        loc_id: Option<String>,
        is_auto_answer: Option<bool>,
        is_mirror: Option<bool>,
    },
    Apply {
        url: Option<String>,
        to_ip: Option<String>,
        from: Option<String>,
        to: Option<String>,
        from_name: Option<String>,
        to_name: Option<String>,
        client_type: Option<String>,
        uuid: Option<String>,
        loc_id: Option<String>,
        is_video: Option<bool>,
        grpc_from: Option<String>,
        grpc_to: Option<String>,

        apply_type: Option<i32>,
        is_all: Option<bool>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Version { url } => {
            println!("'version command' was used, option url is: {url:?}");
            let url_str: String = url.as_deref().unwrap_or("http://[::1]:7880").to_string();
            let mut client = GatewayClient::connect(url_str).await?;
            let request = tonic::Request::new(Null {});
            // println!("Sending request to gRPC Server...");
            let response = client.grpc_version(request).await?;
            println!("RESPONSE={:?}", response);
        }
        Commands::Register {
            url,
            to_ip,
            num,
            client_type,
            ip,
            identity,
            display_name,
            grpc_port,
            http_port,
            loc_id,
            is_auto_answer,
            is_mirror,
        } => {
            println!("'register command' was used, option url is: {url:?}");
            let url_str: String = url.as_deref().unwrap_or("http://[::1]:7880").to_string();
            let mut client = GatewayClient::connect(url_str).await?;
            let request = tonic::Request::new(RegisterRequest {
                to_ip: to_ip.as_deref().unwrap_or("10.0.0.34").to_string(),
                num: num.as_deref().unwrap_or("1100").to_string(),
                client_type: client_type.as_deref().unwrap_or("bed").to_string(),
                ip: ip.as_deref().unwrap_or("192.168.20.19").to_string(),
                identity: identity.as_deref().unwrap_or("-").to_string(),
                display_name: display_name.as_deref().unwrap_or("rust").to_string(),
                grpc_port: grpc_port.unwrap_or(7880),
                http_port: http_port.unwrap_or(0),
                loc_id: loc_id.as_deref().unwrap_or("12160708").to_string(),
                is_auto_answer: is_auto_answer.unwrap_or(false),
                is_mirror: is_mirror.unwrap_or(false),
            });
            // println!("Sending request to gRPC Server...");
            let response = client.grpc_register(request).await?;

            println!(
                "STATUS={:?} MESSAGE={:?}",
                response
                    .get_ref()
                    .base_response
                    .as_ref()
                    .unwrap()
                    .status
                    .to_string(),
                response
                    .get_ref()
                    .base_response
                    .as_ref()
                    .unwrap()
                    .message
                    .to_string(),
            );
        }
        Commands::Apply {
            url,
            to_ip,
            from,
            to,
            from_name,
            to_name,
            client_type,
            uuid,
            loc_id,
            is_video,
            grpc_from,
            grpc_to,
            apply_type,
            is_all,
        } => {
            println!("'register command' was used, option url is: {url:?}");
            let url_str: String = url.as_deref().unwrap_or("http://[::1]:7880").to_string();
            let mut client = GatewayClient::connect(url_str).await?;
            let request = tonic::Request::new(ApplyRequest {
                common_request: Some(CommonRequest {
                    from: from.as_deref().unwrap_or("1100").to_string(),
                    to: to.as_deref().unwrap_or("1012").to_string(),
                    from_name: from_name.as_deref().unwrap_or("rust").to_string(),
                    to_name: to_name.as_deref().unwrap_or("").to_string(),
                    client_type: client_type.as_deref().unwrap_or("bed").to_string(),
                    uuid: uuid.as_deref().unwrap_or("").to_string(),
                    loc_id: loc_id.as_deref().unwrap_or("12160708").to_string(),
                    is_video: is_video.unwrap_or(false),
                    grpc_from: grpc_from.as_deref().unwrap_or("1100").to_string(),
                    grpc_to: grpc_to.as_deref().unwrap_or("1100").to_string(),
                }),
                to_ip: to_ip.as_deref().unwrap_or("10.0.0.34").to_string(),
                apply_type: apply_type.unwrap_or(1),
                is_all: is_all.unwrap_or(false),
            });
            // println!("Sending request to gRPC Server...");
            let response = client.grpc_apply(request).await?;

            println!(
                "STATUS={:?} MESSAGE={:?}",
                response.get_ref().status.to_string(),
                response.get_ref().message.to_string(),
            );
        }
    }

    Ok(())
}
