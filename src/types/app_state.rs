// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::db_pool::DBPool;

#[derive(Clone)]
pub struct AppState {
    pub db: DBPool,
}

impl AppState {
    pub fn new(db_pool: DBPool) -> Self {
        AppState { db: db_pool }
    }
}
