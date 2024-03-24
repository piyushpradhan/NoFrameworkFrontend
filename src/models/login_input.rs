pub struct LoginInput {
    pub username: String,
    pub password: String,
}

impl LoginInput {
    pub fn new(username: String, password: String) -> Self {
        LoginInput { username, password }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}

impl Default for LoginInput {
    fn default() -> Self {
        LoginInput {
            username: String::new(),
            password: String::new(),
        }
    }
}
