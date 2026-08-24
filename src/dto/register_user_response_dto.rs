// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::{Deserialize, Serialize};

use crate::types::base_response::{BaseResponse, ErrorResponse};

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
}
