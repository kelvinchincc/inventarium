// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::{db_pool::DBPool, env_variables::EnvVariables};

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub db: DBPool,
    pub config: EnvVariables,
}

impl AppState {
    pub fn new(db_pool: DBPool, config: EnvVariables) -> Self {
        AppState {
            db: db_pool,
            config,
        }
    }
}
