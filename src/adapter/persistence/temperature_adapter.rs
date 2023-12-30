use anyhow::Result;
use futures::future::join_all;
use sqlx::QueryBuilder;
use crate::adapter::persistence::get_pool;
use crate::application::port::outbound::SaveTemperaturePort;
use crate::domain::temperature::{SaveTemperatureItem};

#[derive(Debug)]
pub struct DbTemperature {
    pub serial_number: String,
    pub temperature: i16,
    pub measure_at: String,
}

impl SaveTemperaturePort {
    pub(crate) async fn save_temperatures(serial_number: String, items: Vec<SaveTemperatureItem>) -> Result<bool> {
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
