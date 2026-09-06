use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct EnvVariables {
    jwt_secret: Vec<u8>,
    jwt_ref_secret: Vec<u8>,
    use_wal_mode: bool,
}

impl EnvVariables {
    pub fn parse_env_configs() -> Result<EnvVariables> {
        let jwt_secret =
            std::env::var("JWT_SECRET").map_err(|_| anyhow!("JWT_SECRET is required"))?;
        let jwt_ref_secret =
            std::env::var("JWT_REF_SECRET").map_err(|_| anyhow!("JWT_REF_SECRET is required"))?;
        let use_wal_mode = std::env::var("USE_WAL_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        Ok(EnvVariables {
            jwt_secret: jwt_secret.as_bytes().to_vec(),
            jwt_ref_secret: jwt_ref_secret.as_bytes().to_vec(),
            use_wal_mode,
        })
    }

    pub fn get_jwt_secret(&self) -> &[u8] {
        &self.jwt_secret
    }

    pub fn get_jwt_ref_secret(&self) -> &[u8] {
        &self.jwt_ref_secret
    }

    pub fn get_sqlite_is_wal_mode(&self) -> bool {
        self.use_wal_mode
    }
}
