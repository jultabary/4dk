use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ArticleIsAlreadyPublished {
    pub article: String,
}

impl Display for ArticleIsAlreadyPublished {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Article [{}] is already published", self.article)
    }
}

impl Error for ArticleIsAlreadyPublished {}

#[derive(Debug)]
pub struct NewsPaperAlreadyExist {
    pub news_paper: String,
}

impl Display for NewsPaperAlreadyExist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NewsPaper [{}] already exist", self.news_paper)
    }
}

impl Error for NewsPaperAlreadyExist {}


#[derive(Debug)]
pub struct NewsPaperDoesNotExist {
    pub news_paper: String,
}

impl Display for NewsPaperDoesNotExist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NewsPaper [{}] does not exist", self.news_paper)
    }
}

impl Error for NewsPaperDoesNotExist {}
