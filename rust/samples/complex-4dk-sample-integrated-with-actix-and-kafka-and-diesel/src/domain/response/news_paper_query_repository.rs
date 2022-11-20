use crate::domain::response::news_paper_response::NewsPaperResponse;

pub trait NewsPaperQueryRepository {
    fn find_all(&self) -> Vec<NewsPaperResponse>;

    fn find_all_even_with_unpublished_article(&self) -> Vec<NewsPaperResponse>;
}

