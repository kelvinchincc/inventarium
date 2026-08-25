// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::dto::login_request_dto::LoginRequestDto;
use crate::dto::login_response_dto::LoginResponseDto;
use crate::service::auth_service::error::AuthServiceError;
use crate::types::jwt_payload::{JWTPayload, JWTTokenPair};
use crate::{
    dto::register_user_request_dto::RegisterUserRequestDto,
    entity::user::User,
    repo::{base_repo::user_repo, sqlite_repo::error::SQLiteRepoError},
    types::db_pool::DBPool,
};
use anyhow::{Context, Result};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};

/// Creates a new user in the database.
///
/// *Parameters:*
/// - `db_pool`: A reference to the database pool.
/// - `user`: A reference to the register user request DTO containing the username, email, and password.
///
/// *Returns:*
/// - `Result<User>`: Returns the created user if successful, or an error, refer error section for more details.
///
/// *Errors:*
/// - `AuthServiceError::UserAlreadyExists`: Returned if a user with the same username already exists in the database.
pub async fn create_user(db_pool: &DBPool, user: &RegisterUserRequestDto) -> Result<User> {
    let user = User::new(
        uuid::Uuid::now_v7().to_string(),
        user.username.clone(),
        user.email.clone(),
        hash_password(user.password.as_str())?,
    );
    log::debug!("Creating user: {:?}", user);
    let created = match user_repo::create_user(db_pool, &user).await {
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

/// Logs in a user by verifying their credentials.
///
/// *Parameters:*
/// - `db`: A reference to the database pool.
/// - `login_dto`: A reference to the login request DTO containing the username and password
///
/// *Returns:*
/// - `Result<()>`: Returns `Ok(())` if the login is successful, or an error, refer error section for more details.
///
/// *Errors:*
/// - `AuthServiceError::BadCredentials`: Returned if the username does not exist or the password is incorrect.
pub async fn login(db: &DBPool, login_dto: &LoginRequestDto) -> Result<LoginResponseDto> {
    let user = user_repo::get_user_by_username(&db, login_dto.username.as_str()).await?;
    if user.is_none() {
        return Err(AuthServiceError::BadCredentials.into());
    }

    let user = user.unwrap();

    match verify_password(&login_dto.password, &user.password_hash) {
        Ok(_) => {
            let auth_token_exp = Utc::now() + chrono::Duration::hours(1);
            let refresh_token_exp = Utc::now() + chrono::Duration::days(14);
            let tokens = generate_jwt_token(
                user.username,
                auth_token_exp,
                refresh_token_exp,
                user.id.clone(),
            )?;
            let mut response = LoginResponseDto::default();
            response.token = tokens.auth;
            response.refresh_token = tokens.refresh;
            Ok(response)
        }
        Err(_) => Err(AuthServiceError::BadCredentials.into()),
    }
}

/// Hashes a password using the Argon2id algorithm.
///
/// *Parameters:*
/// - `password`: A string slice representing the password to be hashed.
///
/// *Returns:*
/// - `Result<String>`: Returns the hashed password as a string if successful, or an error if the hashing process fails.
///
/// *Errors:*
/// - Returns an error if the password hashing process fails.
fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2id = Argon2::default();
    let password_hash = argon2id
        .hash_password(password.as_bytes(), &salt)
        .context("Failed to hash password")?
        .to_string();
    Ok(password_hash)
}

/// Verifies a password against a hashed password using the Argon2id algorithm.
///
/// *Parameters:*
/// - `password`: A string slice representing the password to be verified.
/// - `password_hash`: A string slice representing the hashed password to verify against.
///
/// *Returns:*
/// - `Result<()>`: Returns `Ok(())` if the password matches the hash, or an error if the verification fails.
///
/// *Errors:*
/// - Returns an error if the password verification process fails.
fn verify_password(password: &str, password_hash: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    let matched = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    match matched {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("Password verification failed")),
    }
}

/// Generates a JWT token pair for the given username, expiration time, and seed.
///
/// *Parameters:*
/// - `username`: A string representing the username to be included in the token payload.
/// - `exp`: A `DateTime` representing the expiration time of the token.
/// - `ref_exp`: A `DateTime` representing the expiration time of the refresh token.
/// - `seed`: A string representing the seed to be included in the token payload.
///
/// *Returns:*
/// - `Result<JWTTokenPair>`: Returns a `JWTTokenPair` if the token generation is successful, or an error if it fails.
///
/// *Errors:*
/// - `anyhow::Error`: Returns an error if the token generation fails.
pub fn generate_jwt_token(
    username: String,
    exp: DateTime<Utc>,
    ref_exp: DateTime<Utc>,
    seed: String,
) -> Result<JWTTokenPair> {
    use jsonwebtoken::*;
    let key = EncodingKey::from_secret(b"hello");
    let ref_key = EncodingKey::from_secret(b"secret");
    let payload = JWTPayload::new(username.clone(), exp, seed.clone());
    let ref_payload = JWTPayload::new(username, ref_exp, seed);
    let jwt_tokens = JWTTokenPair {
        auth: encode(&Header::default(), &payload, &key)?,
        refresh: encode(&Header::default(), &ref_payload, &ref_key)?,
    };
    Ok(jwt_tokens)
}

pub mod error {
    use thiserror::Error;

    pub type Username = String;

    #[derive(Debug, Error)]
    pub enum AuthServiceError {
        #[error("User already exists: {0}")]
        UserAlreadyExists(Username),
        #[error("Invalid credentials")]
        BadCredentials,
    }
}
