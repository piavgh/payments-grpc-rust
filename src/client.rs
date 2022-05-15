use payments::ethereum_client::EthereumClient;
use payments::EthPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EthereumClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EthPaymentRequest {
        from_addr: "2B1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_owned(),
        to_addr: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        amount: 1234,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
