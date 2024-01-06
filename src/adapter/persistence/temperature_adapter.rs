use anyhow::Result;
use async_trait::async_trait;
use sqlx::QueryBuilder;
use crate::adapter::persistence::get_pool;
use crate::application::port::outbound::save_temperature_port::SaveTemperaturePort;
use crate::domain::temperature::{SaveTemperatureItem};

#[derive(Debug)]
struct DbTemperature {
    pub serial_number: String,
    pub temperature: i16,
    pub measure_at: String,
}

pub struct SaveTemperatureAdapter;

#[async_trait]
impl SaveTemperaturePort for SaveTemperatureAdapter {
    async fn save_temperatures(&self, serial_number: String, items: Vec<SaveTemperatureItem>) -> Result<bool> {
        // let pool = get_pool().await;
        // for (item) in items {
        //     sqlx::query!("INSERT INTO temperatures(serial_number, temperature, registered_at) VALUES(?, ?, ?) ",
        //         serial_number, item.temperature, item.measure_at)
        //         .execute(pool)
        //         .await?;
        // }

        // let pool = get_pool().await;
        // let jobs = items.iter().map(|item| {
        //     sqlx::query!("INSERT INTO temperatures(serial_number, temperature, registered_at) VALUES(?, ?, ?) ",
        //         serial_number, item.temperature, item.measure_at)
        //         .execute(pool)
        // });
        // join_all(jobs).await;


        let pool = get_pool().await;
        // let q = "INSERT INTO temperatures(serial_number, temperature, registered_at) VALUES(?, ?, ?) ";
        // let jobs = items.iter().map(|item| {
        //     sqlx::query(q)
        //         .bind(serial_number.to_string())
        //         .bind(item.temperature)
        //         .bind(item.measure_at.to_string())
        //         .execute(pool)
        // });
        // join_all(jobs).await;

        let mut qb = QueryBuilder::new("INSERT INTO temperatures(serial_number, temperature, registered_at) ");
        qb.push_values(items, |mut b, new_item| {
            b.push_bind(serial_number.clone())
                .push_bind(new_item.temperature)
                .push_bind(new_item.measure_at);
        });
        qb.build().execute(pool).await?;
        Ok(true)
    }
}
