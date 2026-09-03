// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use poem::web::Data;
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::{
    dto::{
        login_request_dto::LoginRequestDto,
        login_response_dto::LoginResponseType,
        refresh_token_request_dto::RefreshTokenRequestDto,
        refresh_token_response_dto::{RefreshTokenResponseDto, RefreshTokenResponseDtoType},
        register_user_request_dto::RegisterUserRequestDto,
        register_user_response_dto::RegisterUserResponseType,
    },
    gurard::jwt_auth::JWTAuth,
    service::auth_service::{self},
    types::{api_tags::ApiTags, app_state::AppState, jwt_payload::JWTTokenType},
};

pub struct AuthController;

#[OpenApi(tag = "ApiTags::AuthController")]
impl AuthController {
    /// Register user
    #[oai(path = "/public/register", method = "post")]
    pub async fn register_user(
        &self,
        _auth: JWTAuth,
        data: Data<&AppState>,
        body: Json<RegisterUserRequestDto>,
    ) -> RegisterUserResponseType {
        if body.password != body.confirm_password {
            return RegisterUserResponseType::bad_request(Some(
                "Password and confirm password do not match".into(),
            ));
        }

        if body.password.len() < 8 {
            return RegisterUserResponseType::bad_request(Some(
                "Password must be at least 8 characters long".into(),
            ));
        }

        let user = auth_service::create_user(&data.db, &body, None).await;

        match user {
            Ok(u) => u.into(),
            Err(e) => e.into(),
        }
    }

    #[oai(path = "/public/login", method = "post")]
    pub async fn login_user(
        &self,
        app_state: Data<&AppState>,
        body: Json<LoginRequestDto>,
    ) -> LoginResponseType {
        let result = auth_service::login(&app_state.db, &body.0).await;

        match result {
            Ok(r) => r.into(),
            Err(e) => e.into(),
        }
    }

    /// Refresh token endpoint
    #[oai(path = "/public/refresh", method = "post")]
    pub async fn refresh_token(
        &self,
        data: Data<&AppState>,
        body: Json<RefreshTokenRequestDto>,
    ) -> RefreshTokenResponseDtoType {
        let result = auth_service::refresh_session(&data.db, &body.refresh_token).await;

        match result {
            Ok(_) => RefreshTokenResponseDto::default().into(),
            Err(e) => e.into(),
        }
    }

    /// This is protected endpoint
    #[oai(path = "/secured/hello", method = "get")]
    pub async fn protected_hello(&self, auth: JWTAuth) -> PlainText<String> {
        let token = auth.0.token;
        PlainText(format!("Hello, Protected World! Your token is: {}", token))
    }
}
