use tonic::{Request, Response, Status, transport::Server};

use gateway::gateway_server::{Gateway, GatewayServer};
use gateway::{
    AdminCtlRequest, ApplyRequest, BaseResponse, BroadcastRequest, CandidateRequest, Client,
    CommonRequest, ContactRequest, ContactResponse, HangUpRequest, InviteRequest, InviteResponse,
    Null, OfflineRequest, RegisterRequest, RegisterResponse, SdpRequest, SettingRequest,
    StatusRequest, StatusResponse, VersionResponse,
};

// Import the generated proto-rust file into a module
pub mod gateway {
    tonic::include_proto!("_");
}

// Implement the service skeleton for the "Greeter" service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyGateway {}

// Implement the service function(s) defined in the proto
#[tonic::async_trait]
impl Gateway for MyGateway {
    async fn grpc_version(
        &self,
        request: Request<Null>,
    ) -> Result<Response<VersionResponse>, Status> {
        println!("--version--");
        println!("Received request from: {:?}", request);
        let response = VersionResponse {
            base_response: Some(BaseResponse {
                status: 200,
                message: "OK".to_string(),
            }),
            data: "1.0.0".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_invite(
        &self,
        request: Request<InviteRequest>,
    ) -> Result<Response<InviteResponse>, Status> {
        println!("--grpc_invite--");
        println!("Received request from: {:?}", request);
        let response = InviteResponse {
            base_response: Some(BaseResponse {
                status: 200,
                message: "OK".to_string(),
            }),
            uuid: "1.0.0".to_string(),
            is_video: false,
        };

        Ok(Response::new(response))
    }
    async fn grpc_answer(
        &self,
        request: Request<CommonRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("--grpc_answer--");
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 200,
            message: "OK".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_hang_up(
        &self,
        request: Request<HangUpRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("--grpc_hang_up--");
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 200,
            message: "OK".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_candidate(
        &self,
        request: Request<CandidateRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("--grpc_candidate--");
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 200,
            message: "OK".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_sdp(
        &self,
        request: Request<SdpRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("--grpc_sdp--");
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 200,
            message: "OK".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        println!("--grpc_status--");
        println!("Received request from: {:?}", request);
        let response = StatusResponse {
            base_response: Some(BaseResponse {
                status: 200,
                message: "OK".to_string(),
            }),
            status: 0,
        };

        Ok(Response::new(response))
    }
    async fn grpc_register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("--grpc_register--");
        println!("Received request from: {:?}", request);
        let response = RegisterResponse {
            base_response: Some(BaseResponse {
                status: 400,
                message: "DON'T SEVER NODE".to_string(),
            }),
            serve: "".to_string(),
            enable_rnnoise: false,
            i10n: "".to_string(),
            ice_conf: "".to_string(),
            enable_multi: false,
            auto_answer: false,
        };

        Ok(Response::new(response))
    }
    async fn grpc_broadcast(
        &self,
        request: Request<BroadcastRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("--grpc_broadcast--");
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 200,
            message: "OK".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_apply(
        &self,
        request: Request<ApplyRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("--grpc_apply--");
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 400,
            message: "DON'T SEVER NODE".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_setting(
        &self,
        request: Request<SettingRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("--grpc_setting--");
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 400,
            message: "DON'T SEVER NODE".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_contact(
        &self,
        request: Request<ContactRequest>,
    ) -> Result<Response<ContactResponse>, Status> {
        println!("--grpc_contact--");
        println!("Received request from: {:?}", request);
        let response = ContactResponse {
            base_response: Some(BaseResponse {
                status: 400,
                message: "DON'T SEVER NODE".to_string(),
            }),
            client: Some(Client {
                num: "1".to_string(),
                client_type: "1".to_string(),
                ip: "1".to_string(),
                identity: "1".to_string(),
                display_name: "1".to_string(),
                grpc_port: 0,
                http_port: 0,
                loc_id: "1".to_string(),
            }),
        };

        Ok(Response::new(response))
    }
    async fn grpc_admin_ctl(
        &self,
        request: Request<AdminCtlRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("--grpc_admin_ctl--");
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 400,
            message: "DON'T SEVER NODE".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn grpc_offline(
        &self,
        request: Request<OfflineRequest>,
    ) -> Result<Response<BaseResponse>, Status> {
        println!("Received request from: {:?}", request);
        let response = BaseResponse {
            status: 200,
            message: "OK".to_string(),
        };

        Ok(Response::new(response))
    }
}

// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.20.19:7880".parse()?;
    let gateway = MyGateway::default();
    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(GatewayServer::new(gateway))
        .serve(addr)
        .await?;

    Ok(())
}
