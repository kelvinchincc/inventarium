// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: Default::default(),
            username: Default::default(),
            email: Default::default(),
            password_hash: Default::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl User {
    pub fn new(id: String, username: String, email: String, password_hash: String) -> Self {
        User {
            id,
            username,
            email,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
