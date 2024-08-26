# Chinese character reader with Shuttle and Turso

This is a partial port of my dictionary manager to the Shuttle platform, using Turso to host the sqlite dictionary database

Since the Shuttle documentation and examples are more detailed for the Axum framework than for Actix, I chose to switch to Axum, which proved quite easy.

Setting up the database on Turso worked like a charm, simply following the Quickstart guide [https://docs.turso.tech/quickstart](https://docs.turso.tech/quickstart)

The program accesses the online database, both when run locally (cargo shuttle run) and when run from shuttle.

I met two issues:

- I initially used the most recent version of libsql, which was 0.5.0. But this version proved non compatible with the Shuttle stack, so I had to switch back to libsql 0.3.1

- When writing handlers for the Axum framework, the extractors (in my case the Form extractor) should be listed after` the state parameter in the handler parameters.

In both cases, the error diagnostic was very unhelpful, typically :
"the trait bound `fn(Form<CharData>, axum::extract::State<Arc<libsql::Database>>) -> impl Future<Output = impl IntoResponse> {zilist}: Handler<_, _>` is not satisfied"

Additionnaly, when working locally, I cannot work without an internet connection (since I need access to the distant libsql database...)

Deserialization of database rows is available for libsql, through module libsql::de, starting with version 0.3.5, but I did not need it here.

Watch the program on [https://chinese-writer.shuttleapp.rs/](https://chinese-writer.shuttleapp.rs/)
