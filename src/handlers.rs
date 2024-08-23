use crate::dbase;
use crate::forms;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::Form;
use libsql::Database;
use std::sync::Arc;
use tera::Tera;

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

pub async fn index(State(client): State<Arc<Database>>) -> impl IntoResponse {
    let size = dbase::getsize(client).await;
    let mut context = tera::Context::new();
    if size > 0 {
        context.insert("contenu", "Connected to database");
    } else {
        context.insert("contenu", "Could not connect to database");
    }
    let output = TERA.render("index.html", &context);
    Html(output.unwrap())
}

pub async fn size(State(client): State<Arc<Database>>) -> impl IntoResponse {
    // https://stackoverflow.com/questions/669092/sqlite-getting-number-of-rows-in-a-database
    let mut ctx = tera::Context::new();

    let size = dbase::getsize(client).await;
    ctx.insert(
        "content",
        format!("The dictionary presently contains {} entries.", &size).as_str(),
    );
    let output = TERA.render("components/content.html", &ctx);
    Html(output.unwrap())
}

pub async fn getziform() -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    let insert: String = forms::ziform();
    ctx.insert("content", &insert);
    let output = TERA.render("components/content.html", &ctx);
    Html(output.unwrap())
}

pub async fn getpyform() -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    let insert: String = forms::pyform();
    ctx.insert("content", &insert);
    let output = TERA.render("components/content.html", &ctx);
    Html(output.unwrap())
}

pub async fn zilist(
    State(client): State<Arc<Database>>,
    Form(chardata): Form<dbase::CharData>, // caution:the extractor should follow the state
) -> impl IntoResponse {
    let chain = &chardata.carac;
    let first: char = chain.chars().next().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("query", &chain);
    let disp = dbase::list_for_zi(client, format!("{:X}", first as u32)).await;
    ctx.insert("dico", &disp);
    let output = TERA.render("components/zilist.html", &ctx);
    Html(output.unwrap())
}

pub async fn pylist(
    State(client): State<Arc<Database>>,
    Form(chardata): Form<dbase::PinyinData>, // caution:the extractor should follow the state
) -> impl IntoResponse {
    let chain = &chardata.pinyin;
    let mut ctx = tera::Context::new();
    ctx.insert("query", &chain);
    let disp = dbase::list_for_py(client, String::from(chain)).await;
    ctx.insert("dico", &disp);
    let output = TERA.render("components/zilist.html", &ctx);
    Html(output.unwrap())
}

pub async fn cancel() -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("content", "Form canceled");
    let output = TERA.render("components/content.html", &ctx);
    Html(output.unwrap())
}
