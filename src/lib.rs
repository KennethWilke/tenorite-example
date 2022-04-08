use std::collections::HashMap;

use tenorite::{TenoriteTask, async_trait, TenoriteRequest, TenoriteService};

#[derive(Debug, Clone)]
pub enum ExampleRequest {
    Set {
        key: String,
        value: String
    },
    Get {
        key: String
    },
    Delete{
        key: String
    }
}

#[derive(Debug, Clone)]
pub enum ExampleResponse {
    EmptyResponse,
    StringResponse(String)
}

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ExampleError {
    #[error("Unexpected error!")]
    Unexpected,
}

pub struct ExampleTaskConfig {
    pub data: HashMap<String, String>
}

pub struct ExampleTask {}

#[async_trait]
impl TenoriteTask<ExampleRequest, ExampleResponse, ExampleError, ExampleTaskConfig> for ExampleTask {
    async fn task(mut receiver: tenorite::Receiver<TenoriteRequest<ExampleRequest, ExampleResponse, ExampleError>>, mut config: ExampleTaskConfig) {
        println!("[ExampleTask] Task Started");

        while let Some(request) = receiver.recv().await {
            println!("[ExampleTask] Received Request: {:?}", request.request);

            use ExampleRequest::*;
            use ExampleResponse::*;
            let response = match request.request {
                Set{key, value} => {
                    config.data.insert(key, value);
                    EmptyResponse
                },
                Get{key} => {
                    match config.data.get(&key) {
                        Some(value) => StringResponse(value.to_string()),
                        None => EmptyResponse
                    }
                    
                },
                Delete{key} => {
                    config.data.remove(&key);
                    EmptyResponse
                }
            };

            match request.client.send(Ok(response)) {
                Err(_result) => {
                    panic!("Error!!!!!")
                },
                _ => {}
            }
        }
    }
}

pub struct ExampleService{}

impl TenoriteService<ExampleRequest, ExampleResponse, ExampleError, ExampleTask, ExampleTaskConfig> for ExampleService{}