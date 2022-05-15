use tonic::{transport::Server, Request, Response, Status};

use payments::ethereum_server::{Ethereum, EthereumServer};
use payments::{EthPaymentRequest, EthPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct EthereumService {}

#[tonic::async_trait]
impl Ethereum for EthereumService {
    async fn send_payment(
        &self,
        request: Request<EthPaymentRequest>,
    ) -> Result<Response<EthPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = EthPaymentResponse {
            successful: true,
            message: format!("Send {} ETH to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = EthereumService::default();

    Server::builder()
        .add_service(EthereumServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}
