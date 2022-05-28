use ulid::Ulid;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub uid: Option<Ulid>,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn new(name: &str, password: &str) -> Self {
        User {
           uid: Some(Ulid::new()), 
           name: name.into(),
           password: password.into()
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub id: Ulid, 
}


// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub password: String
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct CreateLinkRequest {
    pub url: String
}