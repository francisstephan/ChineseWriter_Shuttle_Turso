use std::sync::Arc;
use axum::response::{ Html, IntoResponse };
use axum::extract::State;
use libsql::Database;
use tera::Tera;
use crate::dbase;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERA: Tera = match Tera::new("vol/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Unable to parse templates: {}", e);
            std::process::exit(1);
        }
    };
}

pub async fn index() -> impl IntoResponse {

    let mut context = tera::Context::new();
    context.insert("contenu","Connected to local server");
    let output = TERA.render("index.html", &context);
    Html(output.unwrap())
}

pub async fn size(State(client): State<Arc<Database>>) -> impl IntoResponse {
    // https://stackoverflow.com/questions/669092/sqlite-getting-number-of-rows-in-a-database
    let mut ctx = tera::Context::new();

    let size = dbase::getsize(client).await;
    /*let metadata = fs::metadata("vol/zidian.db").expect("Failed to read file metadata");
    let time = metadata.modified().unwrap();
    use chrono::prelude::{DateTime, Utc};
    let dt: DateTime<Utc> = time.clone().into(); */
    ctx.insert(
        "content",
        format!(
            "The dictionary presently contains {} entries.",
            &size
        )
        .as_str(),
    );
    let output = TERA.render("components/content.html", &ctx);
    Html(output.unwrap())
}
