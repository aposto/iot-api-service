use std::collections::HashMap;
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use anyhow::Result;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<Pool<MySql>> = OnceCell::new();
pub async fn get_pool() -> &'static Pool<MySql> {
    DB_POOL.get().unwrap()
}


lazy_static! {
    pub static ref GLOBAL_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("key", "value");
        m
    };
}

pub async fn create_mysql_pool(url: String, max_connections: u32) -> Result<()>{
    let pool = MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect(&url).await;

    DB_POOL.set(pool.unwrap()).expect("DB Pool initializing failed..");
    Ok(())
}




