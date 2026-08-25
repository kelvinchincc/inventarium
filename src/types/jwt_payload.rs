// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTPayload {
    pub username: String,
    pub exp: usize,
    pub iat: usize,
    pub seed: String,
}

impl JWTPayload {
    pub fn new(username: String, exp: DateTime<Utc>, seed: String) -> Self {
        JWTPayload {
            username,
            exp: exp.timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            seed,
        }
    }
}

pub struct JWTTokenPair {
    pub auth: String,
    pub refresh: String,
}
