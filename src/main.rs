use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use libsql::Database;
use tower_http::services::ServeDir;

mod dbase;
mod handlers;

#[shuttle_runtime::main]
async fn app(
    #[shuttle_turso::Turso(addr = "libsql://zidian-francisstephan.turso.io", token = "{secrets.TURSO_DB_TOKEN}", local_addr="libsql://zidian-francisstephan.turso.io")]
        client: Database,
)
-> shuttle_axum::ShuttleAxum {
    let client = Arc::new(client);

    let router = Router::new()
        .route("/", get(handlers::index))
        .route("/size", get(handlers::size))
        .with_state(client)
        .nest_service("/assets", ServeDir::new("./vol/assets"));

    Ok(router.into())
}
