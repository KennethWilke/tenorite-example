# Tenorite Example

This project demonstrates the current design pattern around building an asynchronous worker that can be used in other threads in a simple, client-server type of pattern.

The majority of files implement individual parts of a Tenorite service library, while `main.rs` uses the library to create an instance of the service and allows multiple workers to interact with the service in a request/reply pattern.

The example here is a HashMap-as-a-Service

# Example Service Usage

In [main.rs](src/main.rs), the example service and its configuration are instantiated and started.

```rust
let service = ExampleService {};
let config = ExampleTaskConfig {
    data: HashMap::new(),
};
let (task, caller) = service.start_task(32, config);
```

`task` is a `JoinHandle<()>` for the underlying service thread, `caller` is a `TenoriteCaller` that provides a handle that can be cloned to share among multiple threads.

Two tokio tasks kick off that will both use the service. The general flow when combined is:

* Thread 1 reads an unset key, gets empty result
* Thread 2 sets the key
* Thread 1 reads again, gets the result

The implementations of those threads are mostly boring, though it it's worth noting that `caller.clone()` is what makes it easy to share access to the service among threads.

The more interesting bits are the functions that are actually using the handle directly.

The `get_test_key` function is using the "client" interface of the service. The `ExampleRequest` enumeration is the interface to the worker, it provides a simple and flexible structure for this usage pattern. I use the `Get` command and check the response which similarly uses an `ExampleResponse` enumeration for the asynchronous replies from the worker.

```rust
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
```

The top priority for Tenorite is to simplify the usage side of thread-worker services, enabling predictable and friendly component reuse. In this way, asynchronous applications can be built similar to a microservice architecture with much less cognitive load.

So let's build a service!

# Service Construction

Littered across 6 files as if I was an enterprise software developer, are the components that implement the HashMap-as-a-Service.

`config.rs` has a configuration structure for the service. In this case it's just the `HashMap` to... as a service

`error.rs` isn't really used for this example, but Tenorite currently requires an `Error` type and this is it. A useful service could make good use of this though!

`request.rs` contains the request structure, obviously! This is the interface between "client" and "server"

`response.rs` coincidentally has the response structure, and now I'm regretting how modular I made this example... but I do think this would be nice for more complex services

`service.rs` this file has a simple struct that represents The Service. The `impl TenoriteService<spam>` duct tapes it all together

`worker.rs` this is the *actual* worker! It implements `TenoriteWorker<spam>` on a struct that has an async `task(receiver, config)` method. `config` is an instance of the structure that owns the `HashMap` and `receiver` is the tokio `mpsc` receiver to read requests from. In this case it matches through the requests and HashMaps as a service!

This side of the system is very likely to change, hopefully to further simplify building this pattern, and possibly also to add some useful features.
