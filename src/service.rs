use tenorite::TenoriteService;

use crate::{ExampleError, ExampleRequest, ExampleResponse, ExampleTask, ExampleTaskConfig};

pub struct ExampleService {}

impl TenoriteService<ExampleRequest, ExampleResponse, ExampleError, ExampleTask, ExampleTaskConfig>
    for ExampleService
{
}
