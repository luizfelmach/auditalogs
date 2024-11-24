use super::Queue;

pub struct RustClient {}

impl Default for RustClient {
    fn default() -> Self {
        RustClient {}
    }
}

impl Queue for RustClient {
    async fn on_message<F>(&self, callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(crate::core::Data) -> Result<(), Box<dyn std::error::Error>>,
    {
        Ok(())
    }
}
