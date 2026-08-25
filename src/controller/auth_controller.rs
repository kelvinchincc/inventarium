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
        login_request_dto::LoginRequestDto, login_response_dto::LoginResponseType,
        register_user_request_dto::RegisterUserRequestDto,
        register_user_response_dto::RegisterUserResponseType,
    },
    gurard::jwt_auth::JWTAuth,
    service::auth_service::{self, error::AuthServiceError},
    types::{api_tags::ApiTags, app_state::AppState},
};

pub struct AuthController;

#[OpenApi(tag = "ApiTags::AuthController")]
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

        let user = auth_service::create_user(&data.db, &body).await;
        if let Err(e) = user {
            return match e.downcast_ref::<AuthServiceError>() {
                Some(AuthServiceError::UserAlreadyExists(username)) => {
                    RegisterUserResponseType::conflict(Some(format!(
                        "User with username '{}' already exists",
                        username
                    )))
                }
                _ => {
                    log::error!("Error creating user: {}", e);
                    RegisterUserResponseType::internal_server_err(None)
                }
            };
        }

        RegisterUserResponseType::ok(user.unwrap().username)
    }

    #[oai(path = "/public/login", method = "post")]
    pub async fn login_user(
        &self,
        app_state: Data<&AppState>,
        body: Json<LoginRequestDto>,
    ) -> LoginResponseType {
        let result = auth_service::login(&app_state.db, &body.0).await;

        if let Err(e) = result {
            match e.downcast_ref::<AuthServiceError>() {
                Some(AuthServiceError::BadCredentials) => {
                    return LoginResponseType::unauthorized(Some(
                        "Invalid username or password".into(),
                    ));
                }
                _ => {
                    log::error!("Error logging in user: {}", e);
                    return LoginResponseType::internal_server_error(None);
                }
            }
        }

        LoginResponseType::ok(result.unwrap())
    }

    /// This is protected endpoint
    #[oai(path = "/secured/hello", method = "get")]
    pub async fn protected_hello(&self, auth: JWTAuth) -> PlainText<String> {
        let token = auth.0.token;
        PlainText(format!("Hello, Protected World! Your token is: {}", token))
    }
}
