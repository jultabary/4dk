use dddk_security::dddk::security::userinfo::user::{User, UserAuthorization};
use rocket::{Request, request};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

pub struct Token {
    value: UserAuthorization,
}

impl Token {
    pub fn get_user_authorization(&self) -> &UserAuthorization {
        &self.value
    }
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                // check validity
                let user = User::new("login".to_string(), Some("login@email.com".to_string()));
                let user_authorization;
                if token == "token" {
                    user_authorization = UserAuthorization::new(user, vec!["role".to_string()]);
                } else if token == "token_1" {
                    user_authorization = UserAuthorization::new(user, vec!["role_1".to_string()]);
                } else {
                    return Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing));
                }
                Outcome::Success(Token { value: user_authorization })
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}