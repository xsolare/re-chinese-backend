use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Signup {
    pub user: SignupUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Signin {
    pub user: SigninUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SigninUser {
    pub email: String,
    pub password: String,
}