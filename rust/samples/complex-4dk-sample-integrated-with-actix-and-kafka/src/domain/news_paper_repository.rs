use crate::domain::news_paper::NewsPaper;

pub trait NewsPaperRepository {
    fn find_by_name(&self, name: &String) -> Option<NewsPaper>;

    fn save(&self, news_paper: &NewsPaper);

    fn update(&self, news_paper: &NewsPaper);
}