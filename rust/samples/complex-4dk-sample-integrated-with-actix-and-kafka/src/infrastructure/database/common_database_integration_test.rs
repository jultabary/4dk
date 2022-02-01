#[cfg(test)]
pub mod common_database_integration_test {
    use diesel::{PgConnection, RunQueryDsl};
    use crate::schema::articles;
    use crate::schema::news_papers;

    pub fn clean_db(connection: &PgConnection) {
        diesel::delete(articles::table).execute(connection).expect("Fail to DELETE articles");
        diesel::delete(news_papers::table).execute(connection).expect("Fail to DELETE news_papers");
    }
}