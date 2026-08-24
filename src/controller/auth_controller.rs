// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use poem_openapi::{OpenApi, payload::PlainText};

use crate::auth::jwt_auth::JWTAuth;

pub struct AuthController;

#[OpenApi]
impl AuthController {
    /// Hello World!
    #[oai(path = "/public/hello", method = "get")]
    pub async fn hello(&self) -> PlainText<&'static str> {
        PlainText("Hello, World!")
    }

    /// This is protected endpoint
    #[oai(path = "/secured/hello", method = "get")]
    pub async fn protected_hello(&self, auth: JWTAuth) -> PlainText<String> {
        let token = auth.0.token;
        PlainText(format!("Hello, Protected World! Your token is: {}", token))
    }
}
