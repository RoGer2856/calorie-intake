#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EmptyMessage {
    #[serde(skip_serializing)]
    pub _marker: Option<()>,
}

impl EmptyMessage {
    pub fn new() -> Self {
        EmptyMessage { _marker: None }
    }
}
