// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::dto::login_request_dto::LoginRequestDto;
use crate::dto::login_response_dto::LoginResponseDto;
use crate::service::auth_service::error::AuthServiceError;
use crate::types::jwt_payload::{JWTPayload, JWTTokenPair, JWTTokenType};
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

const JWT_SECRET: &[u8] = b"hello";
const JWT_REFRESH_SECRET: &[u8] = b"secret";

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
pub async fn create_user(
    db_pool: &DBPool,
    user: &RegisterUserRequestDto,
    should_reset_password: Option<bool>,
) -> Result<User> {
    let should_reset_password = should_reset_password.unwrap_or(false);
    let user = User::new(
        None,
        user.username.clone(),
        user.email.clone(),
        hash_password(user.password.as_str())?,
        should_reset_password,
    );
    log::debug!("Creating user: {:?}", user);
    match user_repo::create_user(db_pool, &user).await {
        Ok(user) => user,
        Err(e) => match e.downcast_ref::<SQLiteRepoError>() {
            Some(SQLiteRepoError::UserExists) => {
                return Err(AuthServiceError::UserAlreadyExists(user.username.clone()).into());
            }
            None => return Err(e),
        },
    };
    Ok(user)
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
                user.username.clone(),
                auth_token_exp,
                refresh_token_exp,
                user.token_seed.clone(),
                user.ref_token_seed.clone(),
            )?;
            let mut response = LoginResponseDto::default();
            response.token = tokens.auth;
            response.refresh_token = tokens.refresh;
            response.username = user.username;
            Ok(response)
        }
        Err(_) => Err(AuthServiceError::BadCredentials.into()),
    }
}

/// Refreshes a user's session by validating the provided refresh token and generating new authentication and refresh
/// tokens.
///
/// *Parameters:*
/// - `db`: A reference to the database pool.
/// - `ref_token`: A string slice representing the refresh token to be validated and used for generating new tokens.
///
/// *Returns:*
/// - `Result<LoginResponseDto>`: Returns a `LoginResponseDto` containing the new authentication and refresh tokens if
///   successful, or an error if the refresh token is invalid or expired.
///
/// *Errors:*
/// - `AuthServiceError::InvalidToken`: Returned if the provided refresh token is invalid or does not match the user's
///   current refresh token seed.
/// - `AuthServiceError::ExpiredToken`: Returned if the provided refresh token has expired.
pub async fn refresh_session(db: &DBPool, ref_token: &str) -> Result<LoginResponseDto> {
    let payload = decode_jwt_token(ref_token, JWTTokenType::Refresh, db)
        .await
        .map_err(|_| AuthServiceError::InvalidToken)?;
    let user = user_repo::get_user_by_username(db, &payload.username)
        .await?
        .ok_or(AuthServiceError::InvalidToken)?;

    let now = Utc::now();
    let ref_token_exp = payload.exp;
    if ref_token_exp < now.timestamp() {
        return Err(AuthServiceError::ExpiredToken.into());
    }

    let token_exp = now + chrono::Duration::hours(1);
    let refresh_token_exp = now + chrono::Duration::days(14);
    let new_token = generate_jwt_token(
        user.username.clone(),
        token_exp,
        refresh_token_exp,
        user.token_seed,
        user.ref_token_seed,
    )
    .map_err(|_| AuthServiceError::BadCredentials)?;
    let login_response = LoginResponseDto {
        username: user.username,
        token: new_token.auth,
        refresh_token: new_token.refresh,
        expire_at: token_exp.timestamp(),
        refresh_expire_at: refresh_token_exp.timestamp(),
    };

    Ok(login_response)
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
/// - `ref_seed`: A string representing the seed to be included in the refresh token payload.
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
    ref_seed: String,
) -> Result<JWTTokenPair> {
    use jsonwebtoken::*;
    let key = EncodingKey::from_secret(JWT_SECRET);
    let ref_key = EncodingKey::from_secret(JWT_REFRESH_SECRET);
    let payload = JWTPayload::new(username.clone(), exp, seed);
    let ref_payload = JWTPayload::new(username, ref_exp, ref_seed);
    let jwt_tokens = JWTTokenPair {
        auth: encode(&Header::default(), &payload, &key)?,
        refresh: encode(&Header::default(), &ref_payload, &ref_key)?,
    };
    Ok(jwt_tokens)
}

pub async fn decode_jwt_token(
    token: &str,
    token_type: JWTTokenType,
    db_pool: &DBPool,
) -> Result<JWTPayload> {
    use jsonwebtoken::*;
    let secret = match token_type {
        JWTTokenType::Auth => JWT_SECRET,
        JWTTokenType::Refresh => JWT_REFRESH_SECRET,
    };
    let key = DecodingKey::from_secret(secret);
    let validation = Validation::default();
    let token_data = decode::<JWTPayload>(token, &key, &validation)?;

    let now = Utc::now().timestamp();
    let exp = token_data.claims.exp;

    if exp < now {
        log::debug!("Token expired: exp={}, now={}", exp, now);
        return Err(AuthServiceError::ExpiredToken.into());
    }

    let user = user_repo::get_user_by_username(db_pool, &token_data.claims.username)
        .await
        .map_err(|e| {
            log::error!("Failed to get user by username: {}", e);
            AuthServiceError::InvalidToken
        })
        .and_then(|u| u.ok_or(AuthServiceError::InvalidToken))?;

    let current_user_seed = match token_type {
        JWTTokenType::Auth => user.token_seed.as_str(),
        JWTTokenType::Refresh => user.ref_token_seed.as_str(),
    };

    if token_data.claims.seed != current_user_seed {
        log::debug!(
            "Invalid token seed: expected={}, found={}",
            current_user_seed,
            token_data.claims.seed
        );
        return Err(AuthServiceError::InvalidToken.into());
    }

    Ok(token_data.claims)
}

/// Inserts a new admin user into the database if no users currently exist.
pub async fn insert_new_admin_user_if_empty(db_pool: &DBPool) -> Result<()> {
    let user_count = user_repo::get_user_count(db_pool).await?;
    if user_count > 0 {
        return Ok(());
    }

    // No users exist, create a new admin user
    log::info!("No users found in the database. Creating a new admin user.");
    let password = "P@$$w0rd".to_string(); // Temp password, must be changed after first login.
    let default_user = RegisterUserRequestDto {
        username: "admin".to_string(),
        email: "admin@admin.com".to_string(),
        password: password.clone(),
        confirm_password: password,
    };
    create_user(db_pool, &default_user, Some(true)).await?;
    log::info!(concat!(
        "Admin user created with username: 'admin' and password: 'P@$$w0rd'. Please change the password later ",
        "in the console"
    ));

    Ok(())
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
        #[error("Token has expired")]
        ExpiredToken,
        #[error("Invalid token")]
        InvalidToken,
    }
}
