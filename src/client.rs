use twodipd::ipd_client::IpdClient;
use twodipd::JoinRequest;

pub mod twodipd {
    tonic::include_proto!("twodipd");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = IpdClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(JoinRequest {
        player_name: "Tonic".into(),
    });

    let response = client.join(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}