use crate::hybrid::HybridMakeService;
use crate::Handler;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Handler error: {0}")]
    HandlerError(String),
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Service error: {0}")]
    ServerError(String),
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Register error: {0}")]
    RegisterError(String),
}

pub struct Service {
    name: String,
}

impl Service {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn init() -> Result<(), InitError> {
        Ok(())
    }

    pub fn register_handler(&self, handler: Handler) -> Result<(), HandlerError> {
        Ok(())
    }

    pub fn run(
        &self,
    ) -> Result<
        hyper::server::Server<hyper::server::conn::AddrIncoming, HybridMakeService>,
        ServiceError,
    > {
        Err(ServiceError::ServerError("Not implemented".to_string()))
    }
}
