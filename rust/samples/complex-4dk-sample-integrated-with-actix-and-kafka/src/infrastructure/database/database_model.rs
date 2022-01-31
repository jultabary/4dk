use crate::domain::article::Article;
use crate::domain::news_paper::NewsPaper;
use crate::domain::response::news_paper_response::{ArticleResponse, NewsPaperResponse};
use crate::schema::news_papers;
use crate::schema::articles;

#[derive(Queryable, Insertable, Identifiable, PartialEq)]
#[primary_key(name)]
#[table_name = "news_papers"]
pub struct NewsPaperDbModel {
    name: String,
}

#[derive(Queryable, Insertable, Identifiable, Associations, PartialEq)]
#[belongs_to(NewsPaperDbModel, foreign_key="news_paper_name")]
#[primary_key(title)]
#[table_name = "articles"]
pub struct ArticleDbModel {
    title: String,
    body: String,
    news_paper_name: String,
}

impl NewsPaperDbModel {
    pub fn from_domain(news_paper: &NewsPaper) -> (NewsPaperDbModel, Vec<ArticleDbModel>) {
        let news_paper_db_model = NewsPaperDbModel {
            name: news_paper.get_name().clone()
        };
        let articles = news_paper.get_articles().iter()
            .map(|article| { ArticleDbModel::from_domain(article, news_paper.get_name().clone()) })
            .collect::<Vec<ArticleDbModel>>();
        return (news_paper_db_model, articles);
    }
}

impl ArticleDbModel {
    pub fn from_domain(article: &Article, news_paper_name: String) -> ArticleDbModel {
        ArticleDbModel {
            title: article.get_title().clone(),
            body: article.get_body().clone(),
            news_paper_name,
        }
    }
}

impl Article {
    pub fn from_db_model(article_db_model: ArticleDbModel) -> Article {
        Article::new(article_db_model.title.clone(), article_db_model.body.clone())
    }
}

impl NewsPaper {
    pub fn from_db_model(results: (NewsPaperDbModel, Vec<ArticleDbModel>)) -> NewsPaper {
        let articles = results.1.iter()
            .map(|article_db_model| {
                Article::new(article_db_model.title.clone(), article_db_model.body.clone())
            })
            .collect::<Vec<Article>>();
        NewsPaper::reconstitute(
            results.0.name.clone(),
            articles,
        )
    }
}

impl NewsPaperResponse {
    pub fn from_db_model(results: (NewsPaperDbModel, Vec<ArticleDbModel>)) -> NewsPaperResponse {
        let articles = results.1.iter()
            .map(|article_db_model| {
                ArticleResponse::new(article_db_model.title.clone(), article_db_model.body.clone())
            })
            .collect::<Vec<ArticleResponse>>();
        NewsPaperResponse::new(
            results.0.name.clone(),
            articles,
        )
    }
}