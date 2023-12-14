use std::collections::HashMap;
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use anyhow::Result;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

struct DbPool {
    pub pool: Pool<MySql>,
}

/*

https://www.reddit.com/r/learnrust/comments/zb40zr/async_lazy_initialization/
 */

impl DbPool {
    fn new(p: Pool<MySql>) -> DbPool {
        DbPool { pool: p }
    }
}

static DB_POOL: OnceCell<Pool<MySql>> = OnceCell::new();

pub async fn get_pool() -> &'static Pool<MySql> {
    DB_POOL.get().unwrap()
}
// unsafe impl Send for DbPool {}
// unsafe impl Sync for DbPool {}

lazy_static! {
    pub static ref GLOBAL_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("key", "value");
        m
    };
}

lazy_static! {
    // pub static ref DB_POOL: AsyncOnce<Pool<MySql>> = AsyncOnce::new(async {
    //     create_mysql_pool(dotenv::var("DATABASE_URL")?, 10).await?.clone()
    // });
}

// pub async fn get_pool() -> Pool<MySql> {
//     DB_POOL.get()
// }
// pub async fn get_pool() -> Pool<MySql> {
//     DB_POOL.get().await.pool.unwrap()
// }

pub async fn create_mysql_pool(url: String, max_connections: u32) -> Result<()>{
    let pool = MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect(&url).await;
    DB_POOL.set(pool.unwrap());
    Ok(())
}




