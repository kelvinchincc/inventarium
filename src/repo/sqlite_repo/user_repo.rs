// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::borrow::Cow;

use sqlx::sqlite;

use crate::{entity::user::User, repo::sqlite_repo::error::SQLiteRepoError};
use anyhow::Result;

pub async fn create_user<'a>(pool: &sqlite::SqlitePool, user: &'a User) -> Result<Cow<'a, User>> {
    let existing_user =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ? OR email = ?")
            .bind(&user.username)
            .bind(&user.email)
            .fetch_optional(pool)
            .await?;

    if existing_user.is_some() {
        log::debug!("User already exists: {:?}", existing_user.unwrap());
        return Err(SQLiteRepoError::UserExists.into());
    }

    sqlx::query(
        "
        INSERT INTO
          users (
            id,
            username,
            email,
            password_hash,
            token_seed,
            ref_token_seed,
            created_at,
            updated_at
          )
        VALUES
          (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.token_seed)
    .bind(&user.ref_token_seed)
    .bind(&user.created_at)
    .bind(&user.updated_at)
    .execute(pool)
    .await?;

    Ok(Cow::Borrowed(user))
}

pub async fn get_user_by_username(
    pool: &sqlite::SqlitePool,
    username: &str,
) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}
