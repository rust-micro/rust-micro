use greeter::{
    echo_server::{Echo, EchoServer},
    EchoReply, EchoRequest,
};
use micro::{routing::get, Handler, Service};

struct MyEcho;

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn echo(
        &self,
        request: tonic::Request<EchoRequest>,
    ) -> Result<tonic::Response<EchoReply>, tonic::Status> {
        Ok(tonic::Response::new(EchoReply {
            message: format!("Echoing back: {}", request.get_ref().message),
        }))
    }
}

#[tokio::main]
async fn main() {
    let svc = Service::new("rust.micro.srv.greeter");

    let handler = Handler::new();
    handler.add_http_route("/greeter", get(|| async { "Hello World!" }));
    handler.add_grpc(EchoServer::new(MyEcho));

    svc.init();
    svc.register_handler(handler);

    if let Err(e) = svc.run().await {
        eprintln!("server error: {}", e);
    }
}
