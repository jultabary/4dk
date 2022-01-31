use crate::domain::news_paper::NewsPaper;
use crate::domain::response::news_paper_response::NewsPaperResponse;

pub trait NewsPaperRepository {
    fn find_by_name(&self, name: &String) -> Option<NewsPaper>;

    fn save(&self, news_paper: &NewsPaper);

    fn find_all(&self) -> Vec<NewsPaperResponse>;
}