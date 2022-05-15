use payments::ethereum_client::EthereumClient;
use payments::EthPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EthereumClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EthPaymentRequest {
        from_addr: "0x8986dacd4E5E27312f61c0b373C46058FC7B5f87".into(),
        to_addr: "0x8986dacd4E5E27312f61c0b373C46058FC7B5f87".into(),
        amount: 1234,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
