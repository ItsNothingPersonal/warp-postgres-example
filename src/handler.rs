use crate::{
    data::{TodoRequest, TodoResponse},
    db,
    error::Error::DBQuery,
    DBPool,
};
use warp::{http::StatusCode, reject, reply::json, Rejection, Reply};

pub async fn health_handler(db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let db = db::get_db_con(&db_pool).await.map_err(reject::custom)?;

    db.execute("SELECT 1", &[])
        .await
        .map_err(|err| reject::custom(DBQuery(err)))?;

    Ok(StatusCode::OK)
}

pub async fn create_todo_handler(
    body: TodoRequest,
    db_pool: DBPool,
) -> Result<impl Reply, Rejection> {
    Ok(json(&TodoResponse::of(
        db::create_todo(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}
