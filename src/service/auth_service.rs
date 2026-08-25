// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::service::auth_service::error::AuthServiceError;
use crate::{
    dto::register_user_request_dto::RegisterUserRequestDto,
    entity::user::User,
    repo::{base_repo, sqlite_repo::error::SQLiteRepoError},
    types::db_pool::DBPool,
};
use anyhow::Result;

pub async fn create_user(db_pool: &DBPool, user: &RegisterUserRequestDto) -> Result<User> {
    let user = User::new(
        uuid::Uuid::now_v7().to_string(),
        user.username.clone(),
        user.email.clone(),
        user.password.clone(),
    );
    let created = match base_repo::user_repo::create_user(db_pool, &user).await {
        Ok(user) => user,
        Err(e) => match e.downcast_ref::<SQLiteRepoError>() {
            Some(SQLiteRepoError::UserExists) => {
                return Err(AuthServiceError::UserAlreadyExists(user.username.clone()).into());
            }
            None => return Err(e),
        },
    };
    Ok(created.into_owned())
}

pub mod error {
    use thiserror::Error;

    pub type Username = String;

    #[derive(Debug, Error)]
    pub enum AuthServiceError {
        #[error("User already exists: {0}")]
        UserAlreadyExists(Username),
    }
}
