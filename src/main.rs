use grpc_proto::helloworld::{greeter_client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let mut client = GreeterClient::connect(std::env::var("SERVER_URL")?).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Coco".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
