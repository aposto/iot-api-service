


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {

    }
    #[actix_rt::test]
    async fn test_device_select() {
        let rec = sqlx::query("SELECT * from device_groups")
            .fetch_all(get_pool())
            .await?;

        println!("QUERY {:?}", rec);

    }
}