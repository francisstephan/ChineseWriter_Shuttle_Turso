# Chinese character reader with Shuttle and Turso

This is a partial port of my dictionary manager to the Shuttle platform, using Turso to host the sqlite dictionary database

Since the Shuttle documentation and examples are more detailed for the Axum framework than for Actix, I chose to switch to Axum, which proved quite easy.

Setting up the database on Turso worked like a charm, simply following the Quickstart guide [https://docs.turso.tech/quickstart](https://docs.turso.tech/quickstart)

The program also accesses the online database both when run locally (cargo shuttle run) and when run from shuttle.

Watch it on [https://chinese-writer.shuttleapp.rs/](https://chinese-writer.shuttleapp.rs/)
