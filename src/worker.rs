use tenorite::{async_trait, TenoriteRequest, TenoriteWorker};

use crate::{ExampleConfig, ExampleError, ExampleRequest, ExampleResponse};

pub struct ExampleWorker {}

#[async_trait]
impl TenoriteWorker<ExampleRequest, ExampleResponse, ExampleError, ExampleConfig>
    for ExampleWorker
{
    async fn task(
        mut receiver: tenorite::Receiver<
            TenoriteRequest<ExampleRequest, ExampleResponse, ExampleError>,
        >,
        mut config: ExampleConfig,
    ) {
        println!("[ExampleTask] Task Started");

        while let Some(request) = receiver.recv().await {
            println!("[ExampleTask] Received Request: {:?}", request.request);

            use ExampleRequest::*;
            use ExampleResponse::*;
            let response = match request.request {
                Set { key, value } => {
                    config.data.insert(key, value);
                    Ok(EmptyResponse)
                }
                Get { key } => match config.data.get(&key) {
                    Some(value) => Ok(StringResponse(value.to_string())),
                    None => Err(ExampleError::InvalidKey(key)),
                },
                Delete { key } => match config.data.remove(&key) {
                    Some(_) => Ok(EmptyResponse),
                    None => Err(ExampleError::InvalidKey(key)),
                },
            };

            match request.client.send(response) {
                Err(_result) => {
                    panic!("Error!!!!!")
                }
                _ => {}
            }
        }
    }
}
