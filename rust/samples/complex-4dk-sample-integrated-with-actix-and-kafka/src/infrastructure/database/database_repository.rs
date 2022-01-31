use std::env;
use std::rc::Rc;
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, BelongingToDsl, GroupedBy, OptionalExtension};
use dotenv::dotenv;
use crate::domain::news_paper::NewsPaper;
use crate::domain::news_paper_repository::NewsPaperRepository;
use crate::domain::response::news_paper_response::NewsPaperResponse;
use crate::infrastructure::database::database_model::{ArticleDbModel, NewsPaperDbModel};
use crate::schema::articles;
use crate::schema::news_papers;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct NewsPaperRepositoryAdapter {
    pg_connection: Rc<PgConnection>,
}

impl NewsPaperRepositoryAdapter {
    pub fn new(pg_connection: Rc<PgConnection>) -> NewsPaperRepositoryAdapter {
        NewsPaperRepositoryAdapter {
            pg_connection
        }
    }
}

impl NewsPaperRepository for NewsPaperRepositoryAdapter {
    fn find_by_name(&self, name: &String) -> Option<NewsPaper> {
        if let Some(news_paper_found) = news_papers::table
            .filter(news_papers::name.eq(name.clone()))
            .first::<NewsPaperDbModel>(self.pg_connection.as_ref())
            .optional()
            .expect("Error") {
            let articles: Vec<ArticleDbModel> =
                ArticleDbModel::belonging_to(&news_paper_found)
                    .load::<ArticleDbModel>(self.pg_connection.as_ref())
                    .expect("Error");
            return Some(NewsPaper::from_db_model((news_paper_found, articles)));
        }
        None
    }

    fn save(&self, news_paper: &NewsPaper) {
        let news_paper_db_model = NewsPaperDbModel::from_domain(news_paper);
        diesel::insert_into(news_papers::table)
            .values(&news_paper_db_model.0)
            .execute(self.pg_connection.as_ref())
            .expect("Error when saving NewsPaper");
    }

    fn update(&self, news_paper: &NewsPaper) {
        news_paper.get_articles()
            .into_iter()
            .for_each(|article| {
                let article_db_model = ArticleDbModel::from_domain(article, news_paper.get_name().clone());
                diesel::insert_into(articles::table)
                    .values(&article_db_model)
                    .on_conflict(articles::columns::title)
                    .do_update()
                    .set(&article_db_model)
                    .execute(self.pg_connection.as_ref())
                    .expect("Error when upsert article");
            });
    }

    fn find_all(&self) -> Vec<NewsPaperResponse> {
        let news_paper_founds: Vec<NewsPaperDbModel> = news_papers::table
            .load::<NewsPaperDbModel>(self.pg_connection.as_ref())
            .expect("Error");
        let articles =
            ArticleDbModel::belonging_to(&news_paper_founds)
                .load::<ArticleDbModel>(self.pg_connection.as_ref())
                .expect("Error")
                .grouped_by(&news_paper_founds);
        let results: Vec<(NewsPaperDbModel, Vec<ArticleDbModel>)> = news_paper_founds.into_iter().zip(articles).collect::<Vec<_>>();
        let mut news_papers_results = Vec::new() as Vec<NewsPaperResponse>;
        results.into_iter().for_each(|news_papers_with_articles| {
            let news_paper = NewsPaperResponse::from_db_model(news_papers_with_articles);
            news_papers_results.push(news_paper);
        });
        return news_papers_results;
    }
}
