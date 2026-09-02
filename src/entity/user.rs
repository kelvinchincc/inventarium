// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub token_seed: String,
    pub ref_token_seed: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub should_reset_password: bool,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: Default::default(),
            username: Default::default(),
            email: Default::default(),
            password_hash: Default::default(),
            token_seed: Default::default(),
            ref_token_seed: Default::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            should_reset_password: false,
        }
    }
}

impl User {
    pub fn new(
        id: String,
        username: String,
        email: String,
        password_hash: String,
        should_reset_password: bool,
    ) -> Self {
        User {
            id,
            username,
            email,
            password_hash,
            token_seed: Uuid::new_v4().to_string(),
            ref_token_seed: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            should_reset_password,
        }
    }
}
