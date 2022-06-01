
use snaplink::{ service, model as m, data_source::{ Result as DataResult, DataSource, DataSourceError }};

use tokio;
use tracing_subscriber;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::collections::HashMap;
use std::sync::Arc;
use ulid::Ulid;
use async_trait::async_trait;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "example_sessions=debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();

    let client = MemDb::new();
    service::run(Arc::new(Box::new(client))).await;
}

struct MemDb {
    inner: Mutex<MemDbInner>
}

struct MemDbInner {
    users: HashMap<String, m::User>,
    sessions: HashMap<Ulid, m::ValidSession>,
    links: HashMap<Ulid, m::Link>,
}

impl MemDb {
    pub fn new() -> Self {
        let mdb = MemDbInner {
            users: HashMap::new(),
            sessions: HashMap::new(),
            links: HashMap::new(),
        };
        Self {
            inner: Mutex::new(mdb)
        }
    }
}

#[async_trait]
impl DataSource for MemDb {
    async fn create_user(&self, user: &m::User) -> DataResult<m::User> {
        let u = m::User {
            uid: Some(Ulid::new().to_string()),
            ..user.clone()
        };
        let mut guard = self.inner.lock().await;
        guard.users.insert(u.username.clone(), u.clone());
        Ok(u)
    }

    async fn rem_user(&self, username: &str) -> DataResult<()> {
        let mut guard = self.inner.lock().await;
        guard.users.remove(username);
        Ok(())
    }
    async fn create_link(&self, sess: &m::ValidSession, url: &str) -> DataResult<String> {
        let lid = Ulid::new();
        let sid = Ulid::from_string(&sess.id).map_err(DataSourceError::from)?;
        let mut guard = self.inner.lock().await;
        if !guard.sessions.contains_key(&sid) {
            return Err(DataSourceError::Unauthorized("session not found".into()));
        }
        guard.links.insert(lid, m::Link{ url: url.into() });
        Ok(lid.to_string())
    }
    async fn unwrap_link(&self, lid: &str) -> DataResult<String> {
        let lid = Ulid::from_string(lid).map_err(DataSourceError::from)?;
        let mut guard = self.inner.lock().await;
        let resolved = guard.links.get(&lid).ok_or(DataSourceError::NotFound("Link not found".into()))?;
        let url = resolved.url.clone();
        guard.links.remove(&lid);
        Ok(url)

    }
    async fn rem_link(&self, lid: &str) -> DataResult<()> {
        let lid = Ulid::from_string(lid).map_err(DataSourceError::from)?;
        let mut guard = self.inner.lock().await;
        guard.links.remove(&lid);
        Ok(())
    }
    #[must_use]
    async fn validate_session(&self, sess_id: &str) -> DataResult<m::ValidSession> {
        let sid = Ulid::from_string(sess_id).map_err(DataSourceError::from)?;
        let guard = self.inner.lock().await;
        let sess = guard.sessions.get(&sid).ok_or(DataSourceError::Unauthorized("session not found".into()))?;
        Ok(sess.to_owned())
    }
    async fn login(&self, user: &m::User)  -> DataResult<m::ValidSession> {
        let mut guard = self.inner.lock().await;
        let usr = guard.users.get(&user.username).ok_or(DataSourceError::Unauthorized("user not found".into()))?;
        if user.password != usr.password {
            return Err(DataSourceError::Unauthorized("password incorrect".into()));
        }
        let sessid = Ulid::new();
        let sess = m::ValidSession { id: sessid.to_string() };
        guard.sessions.insert(sessid, sess.clone());
        Ok(sess)
    }
}
