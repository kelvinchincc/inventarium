// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anyhow::Result;
use std::borrow::Cow;

use crate::{
    entity::user::User,
    repo::sqlite_repo,
    types::db_pool::DBPool::{self, SQLITE},
};

pub async fn create_user<'a>(pool: &DBPool, user: &'a User) -> Result<Cow<'a, User>> {
    match pool {
        SQLITE(pool) => {
            let result = sqlite_repo::user_repo::create_user(pool, user).await?;
            return Ok(result);
        }
    }
}

pub async fn get_user_by_username<'a>(pool: &DBPool, username: &str) -> Result<Option<User>> {
    match pool {
        SQLITE(pool) => {
            let result = sqlite_repo::user_repo::get_user_by_username(pool, username).await?;
            return Ok(result);
        }
    }
}

pub async fn get_user_count(pool: &DBPool) -> Result<i64> {
    match pool {
        SQLITE(pool) => {
            let result = sqlite_repo::user_repo::get_user_count(pool).await?;
            return Ok(result);
        }
    }
}
