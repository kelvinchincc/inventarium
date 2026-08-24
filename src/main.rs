/***********************************************************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **********************************************************************************************************************/
use anyhow::Result;
use poem_openapi::OpenApiService;

use crate::controller::auth_controller;

mod controller;

#[tokio::main]
async fn main() -> Result<()> {
    init_log();

    let api_service = OpenApiService::new(auth_controller::AuthController, "Inventarium", "1.0.0")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = poem::Route::new()
        .nest("/docs", ui)
        .nest("/api", api_service);
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
