// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::{Deserialize, Serialize};

use crate::types::base_response::{BaseResponse, ErrorResponse};

#[derive(Debug, Serialize, Deserialize, Object, Clone)]
pub struct LoginResponseDto {
    pub username: String,
    pub token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expireAt")]
    pub expire_at: i64,
    #[serde(rename = "refreshExpireAt")]
    pub refresh_expire_at: i64,
}

impl Default for LoginResponseDto {
    fn default() -> Self {
        let now: DateTime<Utc> = Utc::now();
        let expire_at = now.timestamp() + 3600; // 1 hour
        let refresh_expire_at = now.timestamp() + 1209600; // 14 days

        LoginResponseDto {
            username: String::new(),
            token: String::new(),
            refresh_token: String::new(),
            expire_at,
            refresh_expire_at,
        }
    }
}

#[derive(Debug, ApiResponse)]
pub enum LoginResponseType {
    #[oai(status = 200)]
    Ok(Json<BaseResponse<LoginResponseDto>>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorResponse>),
}

impl LoginResponseType {
    pub fn ok(response: LoginResponseDto) -> Self {
        LoginResponseType::Ok(Json(BaseResponse::from(response)))
    }

    pub fn unauthorized(message: Option<String>) -> Self {
        LoginResponseType::Unauthorized(Json(message.unwrap_or("Unauthorized".into()).into()))
    }

    pub fn internal_server_error(message: Option<String>) -> Self {
        LoginResponseType::InternalServerError(Json(
            message.unwrap_or("Internal Server Error".into()).into(),
        ))
    }
}
