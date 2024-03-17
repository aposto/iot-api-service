use crate::domain::temperature::SaveTemperatureItem;

use async_trait::async_trait;

#[async_trait]
pub trait  SaveTemperaturePort {
    async fn save_temperatures(&self, serial_number: String, items: Vec<SaveTemperatureItem>) -> anyhow::Result<bool>;
}