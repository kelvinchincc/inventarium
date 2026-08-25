// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use anyhow::Result;
use poem::{EndpointExt, middleware::Cors};
use poem_openapi::OpenApiService;

use crate::{
    controller::auth_controller,
    types::{app_state::AppState, db_pool::DBPool},
};

mod controller;
mod dto;
mod entity;
mod gurard;
mod repo;
mod service;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    init_log();

    log::info!("Initializing configs...");
    let app_state = AppState::new(setup_db().await?);

    let api_service =
        OpenApiService::new(auth_controller::AuthController, "Inventarium", "1.0.0").server("/api");
    let ui = api_service.swagger_ui();
    let app = poem::Route::new()
        .nest("/docs", ui)
        .nest("/api", api_service)
        .with(Cors::new())
        .data(app_state);
    poem::Server::new(poem::listener::TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}

fn init_log() {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "poem=debug,inventarium=debug");
        }
    }

    tracing_subscriber::fmt::init();
}

async fn setup_db() -> Result<DBPool> {
    log::info!("Connecting database...");
    Ok(setup_sqlite().await?)
}

async fn setup_sqlite() -> Result<DBPool> {
    use sqlx::sqlite::*;

    log::info!("Connecting sqlite database...");

    let options = SqliteConnectOptions::from_str("./inventarium.db")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    migrate_sqlite(&pool).await?;
    Ok(DBPool::SQLITE(pool))
}

async fn migrate_sqlite(pool: &sqlx::sqlite::SqlitePool) -> Result<()> {
    log::info!("Migrating sqlite database...");
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
