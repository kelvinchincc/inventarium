// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTPayload {
    pub username: String,
    pub exp: i64,
    pub iat: i64,
    pub seed: String,
}

impl JWTPayload {
    pub fn new(username: String, exp: DateTime<Utc>, seed: String) -> Self {
        JWTPayload {
            username,
            exp: exp.timestamp(),
            iat: Utc::now().timestamp(),
            seed,
        }
    }
}

pub struct JWTTokenPair {
    pub auth: String,
    pub refresh: String,
}

pub enum JWTTokenType {
    Auth,
    Refresh,
}
