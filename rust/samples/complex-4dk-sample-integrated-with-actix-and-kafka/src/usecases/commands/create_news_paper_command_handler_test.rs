#[cfg(test)]
mod create_news_paper_command_handler_test {
    use std::rc::Rc;
    use dddk_core::dddk::command::command_handler::CommandHandler;
    use crate::domain::error::NewsPaperAlreadyExist;
    use crate::usecases::commands::create_news_paper_command_handler::{CreateNewsPaperCommand, CreateNewsPaperCommandHandler};
    use crate::usecases::events::news_paper_created_event::NewsPaperCreatedEvent;
    use crate::usecases::test::fake_news_paper_repository::fake_repository::FakeNewspaperRepository;

    #[test]
    fn it_should_return_an_error_when_trying_to_create_an_already_create_news_paper() {
        // Given
        let news_paper_name = FakeNewspaperRepository::get_existing_news_paper_name();
        let fake_repository = Rc::new(FakeNewspaperRepository::new());
        let create_news_paper_command_handler = CreateNewsPaperCommandHandler::new(fake_repository);
        let command = CreateNewsPaperCommand { name: news_paper_name };

        // When
        let events = create_news_paper_command_handler.handle(&command);

        // Then
        assert_eq!(true, events.is_err());
        assert_eq!(true, events.err().unwrap().downcast_ref::<NewsPaperAlreadyExist>().is_some());
    }

    #[test]
    fn it_should_create_with_success_new_papers_when_it_does_not_already_exist() {
        // Given
        let fake_repository = Rc::new(FakeNewspaperRepository::new());
        let fake_repository_ref = fake_repository.clone();
        let create_news_paper_command_handler = CreateNewsPaperCommandHandler::new(fake_repository);
        let command = CreateNewsPaperCommand { name: "NewNewsPaper".to_string() };

        // When
        let events = create_news_paper_command_handler.handle(&command);

        // Then
        assert_eq!(false, events.is_err());
        let events = events.unwrap();
        assert_eq!(1, events.len());
        assert_eq!(true, events.get(0).unwrap().as_any().downcast_ref::<NewsPaperCreatedEvent>().is_some());
        assert_eq!(true, fake_repository_ref.get_save_has_been_called().take());
    }

}