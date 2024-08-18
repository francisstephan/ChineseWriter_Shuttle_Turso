use axum::{
    routing::get,
    Router,
};
use libsql::Connection;
use tower_http::services::ServeDir;
use shuttle_axum::ShuttleAxum;
mod handlers;

#[shuttle_runtime::main]
async fn app(
    #[shuttle_turso::Turso(
        addr="libsql://zidian-francisstephan.turso.io",
	    token="{secrets.DB_TURSO_TOKEN}")]
        client: Connection
)-> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(handlers::index))
        .nest_service("/assets", ServeDir::new("./vol/assets"));

    Ok(router.into())
}
