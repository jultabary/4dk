#[cfg(test)]
mod submit_article_command_handler_test {
    use std::rc::Rc;
    use dddk_core::dddk::command::command_handler::CommandHandler;
    use crate::domain::error::{ArticleIsAlreadySubmitted, NewsPaperDoesNotExist};
    use crate::SubmitArticleCommandHandler;
    use crate::usecases::commands::submit_article_command_handler::SubmitArticleCommand;
    use crate::usecases::test::fake_news_paper_repository::fake_news_paper_repository::FakeNewspaperRepository;

    #[test]
    fn it_should_not_submit_an_article_to_a_news_paper_which_not_exist() {
        // Given
        let news_paper_name = "NewsPaper which does not exist".to_string();
        let fake_repository = Rc::new(FakeNewspaperRepository::new());

        let command = SubmitArticleCommand {
            title: "MyTile".to_string(),
            body: "a long time ago in a galaxy far far away".to_string(),
            news_paper_name
        };
        let command_handler = SubmitArticleCommandHandler::new(fake_repository.clone());

        // When
        let err = command_handler.handle(&command);

        // Then
        assert_eq!(true, err.is_err());
        let err = err.err().unwrap();
        assert_eq!(true, err.downcast_ref::<NewsPaperDoesNotExist>().is_some())
    }

    #[test]
    fn it_should_submit_an_article_to_a_news_paper_with_success() {
        // Given
        let news_paper_name = FakeNewspaperRepository::get_existing_news_paper_name();
        let fake_repository = Rc::new(FakeNewspaperRepository::new());

        let command = SubmitArticleCommand {
            title: "MyTile".to_string(),
            body: "a long time ago in a galaxy far far away".to_string(),
            news_paper_name
        };
        let command_handler = SubmitArticleCommandHandler::new(fake_repository.clone());

        // When
        let err = command_handler.handle(&command);

        // Then
        assert_eq!(false, err.is_err());
        assert_eq!(true, fake_repository.get_update_has_been_called().take());
    }

    #[test]
    fn it_should_return_domain_error_from_handler_when_article_already_exist() {
        // Given
        let news_paper_name = FakeNewspaperRepository::get_existing_news_paper_name();
        let fake_repository = Rc::new(FakeNewspaperRepository::new());

        let command = SubmitArticleCommand {
            title: FakeNewspaperRepository::get_associated_article_title(),
            body: "a long time ago in a galaxy far far away".to_string(),
            news_paper_name
        };
        let command_handler = SubmitArticleCommandHandler::new(fake_repository.clone());

        // When
        let err = command_handler.handle(&command);

        // Then
        assert_eq!(true, err.is_err());
        let error = err.err().unwrap();
        assert_eq!(true, error.downcast_ref::<ArticleIsAlreadySubmitted>().is_some());
    }


}