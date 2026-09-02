// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use poem_openapi::{ApiResponse, payload::Json};

use crate::{
    dto::login_response_dto::LoginResponseDto, service::auth_service::error::AuthServiceError,
};

pub type RefreshTokenResponseDto = LoginResponseDto;

#[derive(ApiResponse, Debug)]
pub enum RefreshTokenResponseDtoType {
    #[oai(status = 200)]
    Ok(Json<RefreshTokenResponseDto>),
    #[oai(status = 401)]
    InvalidRefreshToken,
    #[oai(status = 500)]
    InternalServerError,
}

impl RefreshTokenResponseDtoType {
    pub fn ok(dto: RefreshTokenResponseDto) -> Self {
        RefreshTokenResponseDtoType::Ok(Json(dto))
    }

    pub fn invalid_refresh_token() -> Self {
        RefreshTokenResponseDtoType::InvalidRefreshToken
    }
}

impl Default for RefreshTokenResponseDtoType {
    fn default() -> Self {
        RefreshTokenResponseDtoType::InternalServerError
    }
}

impl From<anyhow::Error> for RefreshTokenResponseDtoType {
    fn from(error: anyhow::Error) -> Self {
        match error.downcast_ref() {
            Some(AuthServiceError::InvalidToken) => {
                RefreshTokenResponseDtoType::invalid_refresh_token()
            }
            Some(AuthServiceError::ExpiredToken) => {
                RefreshTokenResponseDtoType::invalid_refresh_token()
            }
            _ => {
                log::error!("Unexpected error: {}", error);
                RefreshTokenResponseDtoType::default()
            }
        }
    }
}
