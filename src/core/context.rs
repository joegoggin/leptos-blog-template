#[cfg(feature = "ssr")]
use super::dotenv::DotEnv;

#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub struct Context {
    pub env: DotEnv,
    pub db: Pool<Postgres>,
}
