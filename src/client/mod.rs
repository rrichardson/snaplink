use serde_json::Value;
use async_trait::async_trait;
use crate::model::{Link, User, Session};
use std::fmt; 

type Result<T> = std::result::Result<T, ClientError>;

#[derive(Debug)]
pub enum ClientError {
    IoError(String),
    Unauthorized(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::IoError(err) => write!(f, "IoError: {}", err),
            ClientError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg)
        }
    }
}

#[async_trait]
pub trait Client: Sync + Send {
    async fn create_user(&self, user: &User) -> Result<User>;
    async fn rem_user(&self, uid: &str) -> Result<()>;
    async fn create_link(&self, sess: &Session, link: &str) -> Result<String>;
    async fn unwrap_link(&self, link: &str) -> Result<String>;
    async fn rem_link(&self, link_id: &str) -> Result<()>;
    async fn validate_session(&self, sess_id: &str) -> Result<Session>;
    async fn login(&self, user: &User)  -> Result<Session>;
}