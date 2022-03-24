pub mod env_key {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const FRONTEND_ORIGIN: &str = "FRONTEND_ORIGIN";
    pub const ADDRESS: &str = "ADDRESS";
}

pub mod error_msg {
    pub const UNAUTHRIZED: &str = "Unauthrized user, please signin.";
}

pub const IGNORE_AUTH_ROUTES: [&str; 4] = ["/api/tags", "/api/users/signup", "/api/users/signin", "/api/users/login"];

pub const AUTHORIZATION: &str = "Authorization";
