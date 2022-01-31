#[cfg(test)]
mod news_paper_test {
    use crate::domain::article::Article;
    use crate::domain::news_paper::NewsPaper;
    use crate::usecases::events::news_paper_created_event::NewsPaperCreatedEvent;

    #[test]
    fn it_should_create_news_paper_with_news_paper_created_event_when_use_new_factory_method() {
        // Given
        let name = "La gazette du sorcier".to_string();

        // When
        let mut news_paper = NewsPaper::new(name);

        // Then
        let events = news_paper.move_generated_events();
        assert_eq!(1, events.len());
        let news_paper_event_opt = events.get(0).unwrap();
        assert_eq!(true, news_paper_event_opt.as_any().downcast_ref::<NewsPaperCreatedEvent>().is_some());
    }

    #[test]
    fn it_should_create_news_paper_with_no_event_when_use_reconstitute_factory_method() {
        // Given
        let name = "La gazette du sorcier".to_string();
        let mut articles = Vec::new();
        articles.push(Article::new("Mon titre".to_string(), "Mon article".to_string()));


        // When
        let mut news_paper = NewsPaper::reconstitute(name, articles);

        // Then
        let events = news_paper.move_generated_events();
        assert_eq!(0, events.len());
        assert_eq!(1, news_paper.get_articles().len());
    }

    #[test]
    fn it_should_submit_with_success_article_when_it_was_not_already_submitted() {
        // Given
        let name = "La gazette du sorcier".to_string();
        let article = Article::new("Mon titre".to_string(), "Mon article".to_string());
        let mut news_paper = NewsPaper::new(name);

        // When
        let err = news_paper.submit(article);

        // Then
        assert_eq!(false, err.is_err());
        assert_eq!(1, news_paper.get_articles().len());
    }

    #[test]
    fn it_should_fail_to_submit_article_when_it_was_already_submited() {
        // Given
        let name = "La gazette du sorcier".to_string();
        let article = Article::new("Mon titre".to_string(), "Mon article".to_string());
        let mut articles = Vec::new();
        articles.push(Article::new("Mon titre".to_string(), "Mon article".to_string()));
        let mut news_paper = NewsPaper::reconstitute(name, articles);

        // When
        let err = news_paper.submit(article);

        // Then
        assert_eq!(true, err.is_err());
    }

}