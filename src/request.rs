#[derive(Debug, Clone)]
pub enum ExampleRequest {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}
