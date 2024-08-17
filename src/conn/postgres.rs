use crate::conf::settings;
use sqlx::{
    postgres::PgPoolOptions,
    Pool,
    Postgres
};


pub async fn get_pool(schema: Option<&str>) -> Result<Pool<Postgres>, sqlx::Error>{
    let dsn: String = match schema{
        Some(s) => format!("{}?options=-csearch_path%3D{}", &settings.database_url, &s),
        None => settings.database_url.clone()
    };
    PgPoolOptions::new()
        .max_connections(settings.database_max_connections as u32)
        .connect(&dsn)
        .await
}


pub async fn create_schema(schema: &str) -> Result<(), sqlx::Error>{
    sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {};", &schema))
        .execute(&get_pool(None).await?)
        .await?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn test_get_pool() {
        env_logger::init();
        let pool = get_pool(None).await.unwrap();
        assert!(
            pool.is_closed() == false,
            "Pool should be open after creation"
        );
    }
   
  }
