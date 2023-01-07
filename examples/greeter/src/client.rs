use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use micro::MicroClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let mut client = MicroClient::new("rust.micro.srv.greeter", GreeterClient).await?;

    // let request = tonic::Request::new(HelloRequest {
    //     name: "Tonic".into(),
    // });
    // let response = client.say_hello(request).await?;
    let response = client
        .request(
            GreeterClient::say_hello,
            HelloRequest {
                name: "Tonic".into(),
            },
        )
        .await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
