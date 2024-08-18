use axum::response::{ Html, IntoResponse };
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


pub async fn index() -> impl IntoResponse {

    let mut context = tera::Context::new();
    context.insert("contenu","Connected to local server");
    let output = TERA.render("index.html", &context);
    Html(output.unwrap())
}
