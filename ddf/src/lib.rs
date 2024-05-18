use sqlx::PgPool;
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::{DDF, NewDDF};

pub mod models;

struct DDFConnection {
    pool: PgPool,
}

impl DDFConnection {
    pub async fn new() -> DDFConnection {
        Self {
            pool: PgPool::connect(env!("DATABASE_URL")).await.unwrap(),
        }
    }
}

async fn get_matching_ddfs(pool: &PgPool, ddf: NewDDF) -> Vec<DDF> {
    let uuid = Uuid::parse_str(&ddf.dce_serial).unwrap();
    sqlx::query_as!(
            DDF,
            r#"
                SELECT id, device_type, sku_number, manufacturer, model, dce_serial FROM ddfs
                WHERE
                    device_type = $1 AND
                    manufacturer = $2 AND
                    dce_serial = $3
            "#,
            ddf.device_type,
            ddf.manufacturer,
            uuid
        )
        .fetch_all(pool)
        .await
        .unwrap()
}

async fn insert_ddf(pool: &PgPool, ddf: NewDDF) {
    let uuid = Uuid::parse_str(&ddf.dce_serial).unwrap();
    sqlx::query!(
            r#"
                INSERT INTO ddfs (device_type, sku_number, manufacturer, model, dce_serial)
                VALUES
                    ($1, $2, $3, $4, $5)
            "#,
            ddf.device_type,
            ddf.sku_number,
            ddf.manufacturer,
            ddf.model,
            uuid
        )
        .execute(pool)
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
