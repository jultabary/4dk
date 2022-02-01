use std::rc::Rc;
use diesel::{ExpressionMethods, PgConnection, BelongingToDsl, RunQueryDsl, QueryDsl, GroupedBy};
use crate::domain::response::news_paper_query_repository::NewsPaperQueryRepository;
use crate::domain::response::news_paper_response::NewsPaperResponse;
use crate::infrastructure::database::database_model::{ArticleDbModel, NewsPaperDbModel};
use crate::schema::articles;
use crate::schema::news_papers;

pub struct NewsPaperQueryRepositoryAdapter {
    pg_connection: Rc<PgConnection>,
}

impl NewsPaperQueryRepositoryAdapter {
    pub fn new(pg_connection: Rc<PgConnection>) -> NewsPaperQueryRepositoryAdapter {
        NewsPaperQueryRepositoryAdapter {
            pg_connection
        }
    }
}

impl NewsPaperQueryRepository for NewsPaperQueryRepositoryAdapter {
    fn find_all(&self) -> Vec<NewsPaperResponse> {
        let news_paper_founds: Vec<NewsPaperDbModel> = news_papers::table
            .load::<NewsPaperDbModel>(self.pg_connection.as_ref())
            .expect("Error");
        let articles =
            ArticleDbModel::belonging_to(&news_paper_founds)
                .filter(articles::columns::is_published.eq(true))
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

    fn find_all_even_with_unpublished_article(&self) -> Vec<NewsPaperResponse> {
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
