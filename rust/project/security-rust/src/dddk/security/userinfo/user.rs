pub struct User {
    login: String,
    email: Option<String>,
}

impl User {
    pub fn new(login: String, email_opt: Option<String>) -> User {
        User {
            login,
            email: email_opt,
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login;
    }

    pub fn get_email(&self) -> &Option<String> {
        &self.email;
    }
}

pub struct UserAuthorization {
    user: User,
    roles: Vec<String>,
}

impl UserAuthorization {
    pub fn new(user: User, roles: Vec<String>) -> UserAuthorization {
        UserAuthorization {
            user,
            roles,
        }
    }
}