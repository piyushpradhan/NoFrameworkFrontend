pub struct User {
    pub id: i32,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl User {
    pub fn new(id: i32, username: String, access_token: String, refresh_token: String) -> Self {
        User {
            id,
            username,
            access_token,
            refresh_token,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            username: String::new(),
            access_token: String::new(),
            refresh_token: String::new(),
        }
    }
}
