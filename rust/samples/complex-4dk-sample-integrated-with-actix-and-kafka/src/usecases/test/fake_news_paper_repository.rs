#[cfg(test)]
pub mod fake_repository {
    use std::cell::RefCell;
    use crate::domain::article::Article;
    use crate::domain::news_paper::NewsPaper;
    use crate::domain::news_paper_repository::NewsPaperRepository;
    use crate::domain::response::news_paper_query_repository::NewsPaperQueryRepository;
    use crate::domain::response::news_paper_response::NewsPaperResponse;

    pub struct FakeNewspaperRepository {
        save_has_been_called: RefCell<bool>,
        update_has_been_called: RefCell<bool>,
    }

    impl FakeNewspaperRepository {
        pub fn new() -> FakeNewspaperRepository {
            FakeNewspaperRepository {
                save_has_been_called: RefCell::new(false),
                update_has_been_called: RefCell::new(false),
            }
        }

        pub fn get_save_has_been_called(&self) -> &RefCell<bool> {
            return &self.save_has_been_called;
        }

        pub fn get_update_has_been_called(&self) -> &RefCell<bool> {
            return &self.update_has_been_called;
        }

        pub fn get_existing_news_paper_name() -> String {
            "NewsPaper".to_string()
        }
        pub fn get_associated_article_title() -> String {
            "ATitle".to_string()
        }
    }

    impl NewsPaperRepository for FakeNewspaperRepository {
        fn find_by_name(&self, name: &String) -> Option<NewsPaper> {
            if name == &FakeNewspaperRepository::get_existing_news_paper_name() {
                Some(NewsPaper::reconstitute(
                    FakeNewspaperRepository::get_existing_news_paper_name(),
                    vec![Article::new(FakeNewspaperRepository::get_associated_article_title(), "".to_string())],
                ))
            } else {
                None
            }
        }

        fn save(&self, _news_paper: &NewsPaper) {
            self.save_has_been_called.replace(true);
        }

        fn update(&self, _news_paper: &NewsPaper) {
            self.update_has_been_called.replace(true);
        }
    }

    pub struct FakeNewsPaperQueryRepository {
        find_all_has_been_called: RefCell<bool>,
        find_all_with_unpublished_article: RefCell<bool>,
    }

    impl NewsPaperQueryRepository for FakeNewsPaperQueryRepository {
        fn find_all(&self) -> Vec<NewsPaperResponse> {
            self.find_all_has_been_called.replace(true);
            Vec::new()
        }

        fn find_all_even_with_unpublished_article(&self) -> Vec<NewsPaperResponse> {
            self.find_all_with_unpublished_article.replace(true);
            Vec::new()
        }
    }

    impl FakeNewsPaperQueryRepository {
        pub fn new() -> FakeNewsPaperQueryRepository {
            FakeNewsPaperQueryRepository {
                find_all_has_been_called: RefCell::new(false),
                find_all_with_unpublished_article: RefCell::new(false),
            }
        }

        pub fn get_find_all_has_been_called(&self) -> &RefCell<bool> {
            return &self.find_all_has_been_called;
        }

        pub fn get_find_all_with_unpublished_article(&self) -> &RefCell<bool> {
            return &self.find_all_with_unpublished_article;
        }
    }

}