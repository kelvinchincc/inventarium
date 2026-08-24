// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use poem_openapi::{
    OpenApi, Tags,
    payload::{Json, PlainText},
};

use crate::{
    auth::jwt_auth::JWTAuth,
    dto::{
        register_user_request_dto::RegisterUserRequestDto,
        register_user_response_dto::{RegisterUserResponseDto, RegisterUserResponseType},
    },
    types::base_response::ErrorResponse,
};

pub struct AuthController;

#[derive(Tags)]
enum tag {
    AuthController,
}

#[OpenApi(tag = "tag::AuthController")]
impl AuthController {
    /// Hello World!
    #[oai(path = "/public/hello", method = "get")]
    pub async fn hello(&self) -> PlainText<&'static str> {
        PlainText("Hello, World!")
    }

    /// Register user
    #[oai(path = "/public/register", method = "post")]
    pub async fn register_user(
        &self,
        body: Json<RegisterUserRequestDto>,
    ) -> RegisterUserResponseType {
        if body.password != body.confirm_password {
            return RegisterUserResponseType::BadRequest(Json(ErrorResponse::from(
                "Password and confirm password do not match",
            )));
        }

        let user = RegisterUserResponseDto::new(body.username.clone());
        RegisterUserResponseType::Ok(Json(user.into()))
    }

    /// This is protected endpoint
    #[oai(path = "/secured/hello", method = "get")]
    pub async fn protected_hello(&self, auth: JWTAuth) -> PlainText<String> {
        let token = auth.0.token;
        PlainText(format!("Hello, Protected World! Your token is: {}", token))
    }
}
