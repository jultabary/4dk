#[cfg(test)]
pub mod fake_news_paper_repository {
    use std::cell::RefCell;
    use crate::domain::article::Article;
    use crate::domain::news_paper::NewsPaper;
    use crate::domain::news_paper_repository::NewsPaperRepository;
    use crate::domain::response::news_paper_response::NewsPaperResponse;

    pub struct FakeNewspaperRepository {
        save_has_been_called: RefCell<bool>,
    }

    impl FakeNewspaperRepository {
        pub fn new() -> FakeNewspaperRepository {
            FakeNewspaperRepository {
                save_has_been_called: RefCell::new(false),
            }
        }

        pub fn get_save_has_been_called(&self) -> &RefCell<bool> {
            return &self.save_has_been_called;
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
                    vec![Article::new(FakeNewspaperRepository::get_associated_article_title(), "".to_string())]
                ))
            } else {
                None
            }
        }

        fn save(&self, news_paper: &NewsPaper) {
            self.save_has_been_called.replace(true);
        }

        fn find_all(&self) -> Vec<NewsPaperResponse> {
            todo!()
        }
    }

}