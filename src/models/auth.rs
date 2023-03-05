use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Signup {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SupertokensUser {
    id: String,
    email: String,
    time_joined: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SignupResponse {
    pub status: String,
    pub user: SupertokensUser,
}
