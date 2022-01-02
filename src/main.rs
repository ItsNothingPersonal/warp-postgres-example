use dotenv::dotenv;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::Filter;

mod data;
mod db;
mod error;
mod handler;

type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;
type Result<T> = std::result::Result<T, warp::Rejection>;

itconfig::config! {
    PORT: u16 => 8000,
    database {
        URL < (
            "postgres://",
            POSTGRES_USERNAME => "postgres",
            ":",
            POSTGRES_PASSWORD: String,
            "@",
            POSTGRES_HOST => "localhost:5432",
            "/",
            POSTGRES_DB => "test",
        ),

        pool {
            MAX_OPEN: u64 => 32,
            MAX_IDLE: u64 => 8,
            TIMEOUT_SECONDS: u64 => 15,
        },
    },

    init_sql: String => "./db.sql",
    TABLE: String => "todo",
    SELECT_FIELDS: String => "id, name, created_at, checked"
}

#[tokio::main]
async fn main() {
    dotenv().expect("dotenv setup to be successful");
    config::init();

    // init logger
    pretty_env_logger::init();

    // init db
    let db_pool = db::create_pool().expect("database pool can be created");
    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    // db filter
    fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
        warp::any().map(move || db_pool.clone())
    }

    // set up routes
    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handler::health_handler);

    let todo = warp::path("todo");
    let todo_routes = todo
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_todos_handler)
        .or(todo
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_todo_handler))
        .or(todo
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_todo_handler))
        .or(todo
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_todo_handler));

    // string all the routes together
    let routes = health_route
        .or(todo_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    // start the api server
    warp::serve(routes)
        .run(([127, 0, 0, 1], config::PORT()))
        .await;
}
