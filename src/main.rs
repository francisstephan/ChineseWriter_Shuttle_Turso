use axum::{routing::get, routing::post, Router};
use libsql::Database;
use std::sync::Arc;
use tower_http::services::ServeDir;

mod dbase;
mod forms;
mod handlers;

#[shuttle_runtime::main]
async fn app(
    #[shuttle_turso::Turso(
        addr = "libsql://zidian-francisstephan.turso.io",
        token = "{secrets.TURSO_DB_TOKEN}",
        local_addr = "libsql://zidian-francisstephan.turso.io"
    )]
    client: Database,
) -> shuttle_axum::ShuttleAxum {
    let client = Arc::new(client);

    let router = Router::new()
        .route("/", get(handlers::index))
        .route("/size", get(handlers::size))
        .route("/getziform", get(handlers::getziform))
        .route("/zilist", post(handlers::zilist))
        .route("/getpyform", get(handlers::getpyform))
        .route("/pylist", post(handlers::pylist))
        .route("/cancel", get(handlers::cancel))
        .with_state(client)
        .nest_service("/assets", ServeDir::new("./vol/assets"));

    Ok(router.into())
}
