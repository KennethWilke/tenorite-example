use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use tenorite::{TenoriteCaller, TenoriteService};
use tenorite_example::*;

#[tokio::main]
async fn main() {
    let service = ExampleService {};
    let config = ExampleTaskConfig {
        data: HashMap::new(),
    };
    let (task, caller) = service.start_task(32, config);

    let thread1_caller = caller.clone();
    tokio::spawn(async move {
        println!("[Thread 1] Started");
        println!("[Thread 1] Requesting key value");
        match get_test_key(thread1_caller.clone()).await {
            Some(value) => {
                println!("[Thread 1] Received Value '{}'", value)
            }
            None => {
                println!("[Thread 1] Received Empty Value")
            }
        }

        println!("[Thread 1] Sleeping for 10 seconds");
        sleep(Duration::from_secs(10)).await;
        println!("[Thread 1] Requesting key value");
        match get_test_key(thread1_caller).await {
            Some(value) => {
                println!("[Thread 1] Received Value '{}'", value)
            }
            None => {
                println!("[Thread 1] Received Empty Value")
            }
        }
        println!("[Thread 1] Done");
    });

    tokio::spawn(async move {
        println!("[Thread 2] Started");
        println!("[Thread 2] Sleeping for 5 seconds");
        sleep(Duration::from_secs(5)).await;
        println!("[Thread 2] Setting key value");
        set_test_key(caller).await;
        println!("[Thread 2] Value set");
        println!("[Thread 2] Done");
    });

    task.await.unwrap();
}

async fn set_test_key(mut caller: TenoriteCaller<ExampleRequest, ExampleResponse, ExampleError>) {
    let key = "test".to_string();
    let value = "weeee".to_string();
    let request = ExampleRequest::Set { key, value };
    match caller.send_request(request).await {
        Err(_error) => {
            eprintln!("error setting test key!");
        }
        _ => {}
    }
}

async fn get_test_key(
    mut caller: TenoriteCaller<ExampleRequest, ExampleResponse, ExampleError>,
) -> Option<String> {
    let key = "test".to_string();
    let request = ExampleRequest::Get { key };
    match caller.send_request(request).await {
        Ok(response) => match response {
            ExampleResponse::StringResponse(value) => Some(value),
            ExampleResponse::EmptyResponse => None,
        },
        Err(_error) => {
            eprintln!("error setting test key!");
            None
        }
    }
}
