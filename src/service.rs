use tenorite::TenoriteService;

use crate::{ExampleConfig, ExampleError, ExampleRequest, ExampleResponse, ExampleWorker};

pub struct ExampleService {}

impl TenoriteService<ExampleRequest, ExampleResponse, ExampleError, ExampleWorker, ExampleConfig>
    for ExampleService
{
}
