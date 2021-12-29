use std::env;
use diesel::{Connection, PgConnection};
use diesel::sql_types::Uuid;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[table_name="organization"]
#[derive(Queryable)]
pub struct Foo {
    pub id: Uuid,
    pub title: String
}
