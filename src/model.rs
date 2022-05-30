use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub uid: Option<String>, 
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        User {
           uid: None,
           username: username.into(),
           password: password.into()
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidSession {
    pub id: String, 
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
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