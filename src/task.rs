use tenorite::{async_trait, TenoriteRequest, TenoriteTask};

use crate::{ExampleError, ExampleRequest, ExampleResponse, ExampleTaskConfig};

pub struct ExampleTask {}

#[async_trait]
impl TenoriteTask<ExampleRequest, ExampleResponse, ExampleError, ExampleTaskConfig>
    for ExampleTask
{
    async fn task(
        mut receiver: tenorite::Receiver<
            TenoriteRequest<ExampleRequest, ExampleResponse, ExampleError>,
        >,
        mut config: ExampleTaskConfig,
    ) {
        println!("[ExampleTask] Task Started");

        while let Some(request) = receiver.recv().await {
            println!("[ExampleTask] Received Request: {:?}", request.request);

            use ExampleRequest::*;
            use ExampleResponse::*;
            let response = match request.request {
                Set { key, value } => {
                    config.data.insert(key, value);
                    EmptyResponse
                }
                Get { key } => match config.data.get(&key) {
                    Some(value) => StringResponse(value.to_string()),
                    None => EmptyResponse,
                },
                Delete { key } => {
                    config.data.remove(&key);
                    EmptyResponse
                }
            };

            match request.client.send(Ok(response)) {
                Err(_result) => {
                    panic!("Error!!!!!")
                }
                _ => {}
            }
        }
    }
}
