use std::collections::HashMap;
use dddk_security::dddk::security::errors::Forbidden;
use dddk_security::dddk::security::userinfo::user::{User, UserAuthorization};
use dddk_security::dddk::security::userinfo::user_authorization_accessor::UserAuthorizationAccessor;

pub struct FakeUserAuthorizationAccessor {}

impl UserAuthorizationAccessor for FakeUserAuthorizationAccessor {
    fn get_user_authorization_from_http_headers(&self, headers: HashMap<String, String>) -> Result<UserAuthorization, Forbidden> {
        let bearer = headers.get("Authorization");
        return if bearer.is_none() {
            Err(Forbidden {})
        } else {
            if bearer.unwrap() == "token" {
                let user = User::new(
                    String::from("login"), Some(String::from("login@email.com")),
                );
                let user_roles = vec![String::from("role")];
                Ok(UserAuthorization::new(user, user_roles))
            } else {
                Err(Forbidden {})
            }
        }
    }

    fn get_user_authorization_accessor_name(&self) -> String {
        String::from("FakeUserAuthorizationAccessor")
    }
}

unsafe impl Sync for FakeUserAuthorizationAccessor {}

unsafe impl Send for FakeUserAuthorizationAccessor {}
