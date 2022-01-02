use crate::data::{Todo, TodoRequest};
use crate::error::Error::DBQuery;
use crate::error::Error::{DBInit, DBPoolConnection};
use crate::{config, DBCon, DBPool};

use chrono::{DateTime, Utc};
use mobc::Pool;
use mobc_postgres::tokio_postgres::Row;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls};

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str(&config::database::URL())?;

    let manager = PgConnectionManager::new(config, NoTls);

    Ok(Pool::builder()
        .max_open(config::database::pool::MAX_OPEN())
        .max_idle(config::database::pool::MAX_IDLE())
        .get_timeout(Some(Duration::from_secs(
            config::database::pool::TIMEOUT_SECONDS(),
        )))
        .build(manager))
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon, crate::error::Error> {
    db_pool.get().await.map_err(DBPoolConnection)
}

pub async fn init_db(db_pool: &DBPool) -> Result<(), crate::error::Error> {
    let init_file = fs::read_to_string(config::init_sql())?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInit)?;
    Ok(())
}

pub async fn create_todo(db_pool: &DBPool, body: TodoRequest) -> Result<Todo, crate::error::Error> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (name) VALUES ($1) RETURNING *",
        config::TABLE()
    );
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQuery)?;

    Ok(row_to_todo(&row))
}

fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let checked: bool = row.get(3);

    Todo {
        id,
        name,
        created_at,
        checked,
    }
}
