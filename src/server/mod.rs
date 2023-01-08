use http::{Request, Response};
use hyper::Body;
use std::convert::Infallible;
use tonic::body::BoxBody;
use tonic::{server::NamedService, transport::Server};
use tower_service::Service;

#[derive(PartialEq)]
pub enum MicroStatus {
    NotInitialized,
    Initialized,
    Running,
}

#[derive(PartialEq)]
pub enum MicroError {
    NotInitialized,
    AlreadyInitialized,
    AlreadyRunning,
}

struct MicroIpAddr(String);

pub struct Micro<S> {
    _service_name: String,
    status: MicroStatus,
    ip: Option<MicroIpAddr>,
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
            _service_name: service_name.to_string(),
            status: MicroStatus::NotInitialized,
            ip: None,
            handler: None,
        }
    }

    pub async fn init(&mut self) -> Result<(), MicroError> {
        if self.status != MicroStatus::NotInitialized {
            return Err(MicroError::AlreadyInitialized);
        }

        // TODO register service in etcd
        // TODO generate port and lookup in etcd, if it is already in use, generate another one

        self.status = MicroStatus::Initialized;

        Ok(())
    }

    pub fn register_handler(&mut self, handler: S) -> &mut Self {
        self.handler = Some(handler);
        self
    }

    pub async fn run(mut self) -> Result<(), MicroError> {
        if self.status != MicroStatus::Initialized {
            return Err(MicroError::NotInitialized);
        }

        self.status = MicroStatus::Running;

        let builder = Server::builder()
            .add_service(std::mem::take(&mut self.handler).expect("No handler given"));

        builder
            .serve(self.ip.unwrap().0.parse().unwrap())
            .await
            .expect("Failed to run server");

        Ok(())
    }
}
