use etcd_client::Client;
use http::{Request, Response};
use hyper::Body;
use portpicker::pick_unused_port;
use std::convert::Infallible;
use tonic::body::BoxBody;
use tonic::{server::NamedService, transport::Server};
use tower_service::Service;

#[derive(PartialEq, Debug)]
pub enum MicroStatus {
    NotInitialized,
    Initialized,
    Running,
}

#[derive(PartialEq, Debug)]
pub enum MicroError {
    NotInitialized,
    AlreadyInitialized,
    AlreadyRunning,
}

pub struct Micro<S> {
    service_name: String,
    status: MicroStatus,
    handler: Option<S>,
}

impl<S> Micro<S>
where
    S: Service<Request<Body>, Response = Response<BoxBody>, Error = Infallible>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            status: MicroStatus::NotInitialized,
            handler: None,
        }
    }

    /// Registers a handler for the service.
    /// If there is already a handler registered, it will be overwritten.
    /// Because of this, it is not possible to register multiple handlers, so this
    /// enforces the single responsibility principle for microservices.
    pub fn register_handler(mut self, handler: S) -> Self {
        if self.status == MicroStatus::NotInitialized {
            self.status = MicroStatus::Initialized;
        }
        self.handler = Some(handler);
        self
    }

    /// Starts the service.
    /// It fails, if there were no handler registered before.
    ///
    /// # Examples
    ///
    // ```
    // use micro::Micro;
    // use hello_world::greeter_server::{Greeter, GreeterServer};
    //
    //
    // Micro::new("rust.micro.srv.greeter")
    //         .register_handler(GreeterServer::new(MyGreeter::default()))
    //         .run()
    //         .await
    //         .expect("Failed to run service");
    // ```

    pub async fn run(mut self) -> Result<(), MicroError> {
        if self.status != MicroStatus::Initialized {
            return Err(MicroError::NotInitialized);
        }

        self.status = MicroStatus::Running;

        let builder = Server::builder()
            .add_service(std::mem::take(&mut self.handler).expect("No handler given"));

        let mut client = Client::connect(["localhost:2379"], None)
            .await
            .expect("Failed to connect to etcd");

        let resp = client
            .get(self.service_name.clone(), None)
            .await
            .expect("Failed to get service");

        let port: u16 = if resp.count() == 0 {
            let port = pick_unused_port().expect("No ports free");

            client
                .put(self.service_name.clone(), port.to_string(), None)
                .await
                .expect("Failed to register service in etcd");

            port
        } else {
            resp.kvs()
                .first()
                .expect("No service found")
                .value_str()
                .expect("Failed to parse service port")
                .parse()
                .expect("Failed to parse service port")
        };

        println!("Starting service on port {}", port);
        builder
            .serve(
                format!("0.0.0.0:{}", port)
                    .parse()
                    .expect("Failed to parse port to serve"),
            )
            .await
            .expect("Failed to run server");

        Ok(())
    }
}
