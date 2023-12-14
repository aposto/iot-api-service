
use sqlx::{MySql, Pool};
pub struct AppContext {
    pool: Pool<MySql>,


}