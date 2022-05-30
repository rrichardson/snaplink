use async_trait::async_trait;
use crate::model::{User, ValidSession};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DataSourceError>;

#[derive(Debug, Error)]
pub enum DataSourceError {
    #[error("IO Error: {0}")]
    IoError(String),
    #[error("UNAUTHORIZED! {0}", )]
    Unauthorized(String),
    #[error("Decode error: {0}")]
    DecodeError(#[from] ulid::DecodeError),
    #[error("Not Found: {0}")]
    NotFound(String)
}

#[async_trait]
pub trait DataSource: Sync + Send {
    async fn create_user(&self, user: &User) -> Result<User>;
    async fn rem_user(&self, username: &str) -> Result<()>;
    async fn create_link(&self, sess: &ValidSession, link: &str) -> Result<String>;
    async fn unwrap_link(&self, link: &str) -> Result<String>;
    async fn rem_link(&self, link_id: &str) -> Result<()>;
    #[must_use]
    async fn validate_session(&self, sess_id: &str) -> Result<ValidSession>;
    async fn login(&self, user: &User)  -> Result<ValidSession>;
}