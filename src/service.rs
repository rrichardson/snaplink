use crate::client::{Client, ClientError};
use crate::model::*;
use async_trait::async_trait;
use std::sync::Arc;
use std::ops::Deref;
use axum::{
    body,
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    extract::{Extension, FromRequest, RequestParts, Path},
    http::Response,
    Json, Router,
};
use std::net::SocketAddr;

#[derive(Clone)]
struct ClientWrapper (Arc<Box<dyn Client>>);

#[derive(Clone)]
struct SessionWrapper (Session, ClientWrapper);

impl Deref for ClientWrapper {
    type Target = Box<dyn Client>;

    fn deref(&self) -> &Box<dyn Client> {
        &*self.0
    }
}

impl IntoResponse for ClientError {
    fn into_response(self) -> axum::response::Response {
        Response::builder().status(500).body(body::boxed(self.to_string())).unwrap()
    }
}

pub async fn run(client: Arc<Box<dyn Client>>) {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/login", post(login))
        .route("/link", post(create_link))
        .route("/link/:lid", get(fetch_link))
        .layer(Extension(client));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[async_trait]
impl<B> FromRequest<B> for SessionWrapper 
where
    B: Send 
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(client) = Extension::<ClientWrapper>::from_request(req)
            .await
            .map_err(internal_error)?;
        let seskey = req.headers().get("Authorization").ok_or((StatusCode::UNAUTHORIZED, "No Authorization Header".into()))?;
        let session = client.validate_session(seskey.to_str().unwrap()).await.map_err(|err| (StatusCode::UNAUTHORIZED, err.to_string()))?;
        Ok(SessionWrapper(session, client.clone()))
    }
}

#[async_trait]
impl<B> FromRequest<B> for ClientWrapper 
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(client) = Extension::<ClientWrapper>::from_request(req)
            .await
            .map_err(internal_error)?;
        Ok(client.clone())
    }
}
// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUserRequest>,
    client: ClientWrapper,
) -> impl IntoResponse {
    // insert your application logic here
    let u = User::new( payload.name.as_str(), payload.password.as_str());
    let user = client.create_user(&u).await.unwrap();
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn create_link(
    Json(payload): Json<CreateLinkRequest>,
    SessionWrapper(sess, client): SessionWrapper, 
) -> Result<impl IntoResponse, ClientError> {
   // insert your application logic here
   let lid = client.create_link(&sess, payload.url.as_str()).await.unwrap();
   // this will be converted into a JSON response
   // with a status code of `201 Created`
   Ok((StatusCode::CREATED, lid))
}

async fn login(
    Json(payload): Json<LoginRequest>,
    client: ClientWrapper, 
) -> Result<impl IntoResponse, ClientError> {
    let u = User::new(payload.username.as_str(), payload.password.as_str());
    let user = client.create_user(&u).await?;
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::OK, Json(user)))
}

async fn fetch_link(
    Path(lid): Path<String>,
    client: ClientWrapper, 
) -> Result<impl IntoResponse, ClientError> {
    let link = client.unwrap_link(lid.as_str()).await?;
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::OK, Json(link)))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
