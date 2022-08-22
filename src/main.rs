//! Json server for mock testing

#![doc = include_str!("./docs/hand.md")]

use axum::Extension;
use clap::Parser;
use db::Database;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry};

mod app;
mod db;
mod extends;
mod models;
mod service;
mod template;
mod util;

#[cfg(test)]
mod tests;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = include_str!("./docs/help.md"))]
struct Args {
    #[clap(short, long, value_parser, default_value = "./static/db.json")]
    config: String,

    #[clap(short, long, value_parser, default_value_t = 8080)]
    port: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    Registry::default().with(fmt::layer()).init();
    tracing::info!("starting json server at port {}", args.port);

    let addr: SocketAddr = format!("127.0.0.1:{}", args.port).parse().unwrap();
    let mut db = Database::new();

    db.init(&args.config);

    let proxy = app::proxy(&db);
    let app = app::create(&db).merge(proxy).layer(Extension(db));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
