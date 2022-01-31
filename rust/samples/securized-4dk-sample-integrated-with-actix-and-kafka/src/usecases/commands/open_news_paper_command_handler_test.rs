#[cfg(test)]
mod OpenNewsPaperCommandHandlerTest {
    use std::cell::RefCell;
    use std::rc::Rc;
    use dddk_core::dddk::command::command_handler::CommandHandler;
    use crate::domain::error::NewsPaperAlreadyExist;
    use crate::domain::news_paper::NewsPaper;
    use crate::domain::news_paper_repository::NewsPaperRepository;
    use crate::domain::response::news_paper_response::NewsPaperResponse;
    use crate::OpenNewsPaperCommandHandler;
    use crate::usecases::commands::open_news_paper_command_handler::OpenNewsPaperCommand;
    use crate::usecases::events::new_news_paper_opened_event::NewNewsPaperOpenedEvent;

    struct FakeNewspaperRepository {
        save_has_been_called: RefCell<bool>,
    }

    impl FakeNewspaperRepository {
        fn new() -> FakeNewspaperRepository {
            FakeNewspaperRepository {
                save_has_been_called: RefCell::new(false),
            }
        }
    }

    impl NewsPaperRepository for FakeNewspaperRepository {
        fn find_by_name(&self, name: &String) -> Option<NewsPaper> {
            if name == &"NewsPaper".to_string() {
                Some(NewsPaper::new("NewsPaper".to_string()))
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

    #[test]
    fn it_should_return_an_error_when_trying_to_open_an_already_open_news_paper() {
        // Given
        let fake_repository = Rc::new(FakeNewspaperRepository::new());
        let open_news_paper_command_handler = OpenNewsPaperCommandHandler::new(fake_repository);
        let command = OpenNewsPaperCommand { name: "NewsPaper".to_string() };

        // When
        let events = open_news_paper_command_handler.handle(&command);

        // Then
        assert_eq!(true, events.is_err());
        assert_eq!(true, events.err().unwrap().downcast_ref::<NewsPaperAlreadyExist>().is_some());
    }

    #[test]
    fn it_should_open_with_success_new_papers_when_it_does_not_already_exist() {
        // Given
        let fake_repository = Rc::new(FakeNewspaperRepository::new());
        let fake_repository_ref = fake_repository.clone();
        let open_news_paper_command_handler = OpenNewsPaperCommandHandler::new(fake_repository);
        let command = OpenNewsPaperCommand { name: "NewNewsPaper".to_string() };

        // When
        let events = open_news_paper_command_handler.handle(&command);

        // Then
        assert_eq!(false, events.is_err());
        let events = events.unwrap();
        assert_eq!(1, events.len());
        assert_eq!(true, events.get(0).unwrap().as_any().downcast_ref::<NewNewsPaperOpenedEvent>().is_some());
        assert_eq!(true, fake_repository_ref.save_has_been_called.take());
    }

}