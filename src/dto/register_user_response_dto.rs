// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::{Deserialize, Serialize};

use crate::{
    entity::user::User,
    service::auth_service::error::AuthServiceError,
    types::base_response::{BaseResponse, ErrorResponse},
};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct RegisterUserResponseDto {
    pub username: String,
    pub success: bool,
}

impl RegisterUserResponseDto {
    pub fn new(username: String) -> Self {
        Self {
            username,
            success: true,
        }
    }
}

#[derive(Debug, ApiResponse)]
pub enum RegisterUserResponseType {
    #[oai(status = 200)]
    Ok(Json<BaseResponse<RegisterUserResponseDto>>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorResponse>),
}

impl RegisterUserResponseType {
    pub fn ok(username: String) -> Self {
        Self::Ok(Json(BaseResponse::from(RegisterUserResponseDto {
            username,
            success: true,
        })))
    }

    pub fn bad_request(msg: Option<String>) -> Self {
        let msg = msg.unwrap_or_else(|| "Bad request".to_string());
        Self::BadRequest(Json(ErrorResponse::from(msg)))
    }

    pub fn conflict(msg: Option<String>) -> Self {
        let msg = msg.unwrap_or_else(|| "Conflict".to_string());
        Self::Conflict(Json(ErrorResponse::from(msg)))
    }

    pub fn internal_server_err(msg: Option<String>) -> Self {
        let msg = msg.unwrap_or_else(|| "Internal server error".to_string());
        Self::InternalServerError(Json(ErrorResponse::from(msg)))
    }
}

impl Default for RegisterUserResponseType {
    fn default() -> Self {
        Self::internal_server_err(None)
    }
}

impl From<User> for RegisterUserResponseType {
    fn from(dto: User) -> Self {
        Self::ok(dto.username)
    }
}

impl From<anyhow::Error> for RegisterUserResponseType {
    fn from(error: anyhow::Error) -> Self {
        match error.downcast_ref::<AuthServiceError>() {
            Some(AuthServiceError::UserAlreadyExists(username)) => {
                RegisterUserResponseType::conflict(Some(format!(
                    "User with username '{}' already exists",
                    username
                )))
            }
            _ => {
                log::error!("Error creating user: {}", error);
                RegisterUserResponseType::internal_server_err(None)
            }
        }
    }
}
