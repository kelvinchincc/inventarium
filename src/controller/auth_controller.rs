/***********************************************************************************************************************
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **********************************************************************************************************************/
use poem_openapi::{OpenApi, payload::PlainText};

pub struct AuthController;

#[OpenApi]
impl AuthController {
    /// Hello World!
    #[oai(path = "/hello", method = "get")]
    pub async fn hello(&self) -> PlainText<&'static str> {
        PlainText("Hello, World!")
    }
}
