use std::collections::HashMap;
use crate::dddk::security::errors::Forbidden;
use crate::dddk::security::userinfo::user::UserAuthorization;

pub trait UserAuthorizationAccessor {
    fn get_user_authorization_from_http_headers(headers: HashMap<String, String>) -> Result<UserAuthorization, Forbidden>;

    fn get_user_authorization_accessor_name() -> String;
}