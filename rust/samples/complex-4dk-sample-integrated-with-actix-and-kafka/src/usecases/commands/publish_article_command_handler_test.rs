#[cfg(test)]
mod publish_article_command_handler_test {
    use std::rc::Rc;
    use dddk_core::dddk::command::command_handler::CommandHandler;
    use crate::domain::error::{ArticleDoesNotExist, NewsPaperDoesNotExist};
    use crate::usecases::commands::publish_article_command_handler::{PublishArticleCommand, PublishArticleCommandHandler};
    use crate::usecases::events::article_has_been_published_event::ArticleHasBeenPublishedEvent;
    use crate::usecases::test::fake_news_paper_repository::fake_news_paper_repository::FakeNewspaperRepository;

    #[test]
    fn it_should_fail_to_publish_an_article_if_news_paper_does_not_exist() {
        // Given
        let fake_repository = Rc::new(FakeNewspaperRepository::new());
        let command_handler = PublishArticleCommandHandler::new(fake_repository.clone());
        let command = PublishArticleCommand { news_paper_name: "Article".to_string(), article_title: "Title".to_string() };

        // When
        let events = command_handler.handle(&command);

        // Then
        assert_eq!(true, events.is_err());
        let error = events.err().unwrap();
        assert_eq!(true, error.downcast_ref::<NewsPaperDoesNotExist>().is_some());
    }

    #[test]
    fn it_should_forward_core_domain_error() {
        // Given
        let fake_repository = Rc::new(FakeNewspaperRepository::new());
        let command_handler = PublishArticleCommandHandler::new(fake_repository.clone());
        let command = PublishArticleCommand {
            news_paper_name: FakeNewspaperRepository::get_existing_news_paper_name(),
            article_title: "Fake Title".to_string(),
        };

        // When
        let events = command_handler.handle(&command);

        // Then
        assert_eq!(true, events.is_err());
        let error = events.err().unwrap();
        assert_eq!(true, error.downcast_ref::<ArticleDoesNotExist>().is_some());
    }

    #[test]
    fn it_should_publish_article_with_success() {
        // Given
        let fake_repository = Rc::new(FakeNewspaperRepository::new());
        let command_handler = PublishArticleCommandHandler::new(fake_repository.clone());
        let command = PublishArticleCommand {
            news_paper_name: FakeNewspaperRepository::get_existing_news_paper_name(),
            article_title: FakeNewspaperRepository::get_associated_article_title(),
        };

        // When
        let events = command_handler.handle(&command);

        // Then
        assert_eq!(false, events.is_err());
        let events = events.unwrap();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();
        assert_eq!(true, event.as_any().downcast_ref::<ArticleHasBeenPublishedEvent>().is_some());
        assert_eq!(true, fake_repository.get_update_has_been_called().take());
    }
}