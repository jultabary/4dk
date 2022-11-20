#[cfg(test)]
mod database_repository_test {
    use std::rc::Rc;
    use diesel::PgConnection;
    use crate::domain::article::Article;
    use crate::domain::news_paper::NewsPaper;
    use crate::domain::news_paper_repository::NewsPaperRepository;
    use crate::domain::response::news_paper_query_repository::NewsPaperQueryRepository;
    use crate::infrastructure::database::common_database_integration_test::common_database_integration_test::clean_db;
    use crate::infrastructure::database::database_query_repository::NewsPaperQueryRepositoryAdapter;
    use crate::infrastructure::database::database_repository::{establish_connection, NewsPaperRepositoryAdapter};

    #[test]
    fn it_should_be_ok() {
        // Given
        let connection = Rc::new(establish_connection());
        clean_db(&connection);
        it_should_save_with_success_given_news_paper(connection.clone());
        it_should_update_news_paper_when_adding_a_new_article(connection.clone());
        it_should_update_article_when_it_has_been_published(connection.clone());
        add_some_data(connection.clone());
        it_should_find_all_news_papers_with_all_articles(connection.clone());
        it_should_find_all_news_papers_with_published_articles(connection.clone());
    }

    fn add_some_data(connection: Rc<PgConnection>) {
        // Given
        let news_paper_repository = NewsPaperRepositoryAdapter::new(connection);
        let mut news_paper = NewsPaper::new("Lemonde".to_string());
        let article = Article::new("Une voiture volante a été vu".to_string(), "Une voiture volante bleu a été aperçu au-dessus de Londres".to_string());
        news_paper_repository.save(&news_paper);
        let _result = news_paper.submit(article);
        news_paper_repository.update(&news_paper);
    }

    fn it_should_save_with_success_given_news_paper(connection: Rc<PgConnection>) {
        // Given
        let news_paper_name = "LaGazetteDuSorcier".to_string();
        let news_paper_repository = NewsPaperRepositoryAdapter::new(connection);
        let news_paper = NewsPaper::new(news_paper_name.clone());

        // When
        news_paper_repository.save(&news_paper);

        // Then
        let option = news_paper_repository.find_by_name(&news_paper_name);
        assert_eq!(true, option.is_some());
        let news_paper = option.unwrap();
        assert_eq!(news_paper.get_name(), &news_paper_name);
    }

    fn it_should_update_news_paper_when_adding_a_new_article(connection: Rc<PgConnection>) {
        // Given
        let news_paper_name = "LaGazetteDuSorcier".to_string();
        let article = Article::new("Voldemort a été vaincu".to_string(), "Harry Potter a vaincu le seigneur des ténèbres !!".to_string());
        let news_paper_repository = NewsPaperRepositoryAdapter::new(connection);
        let mut news_paper = news_paper_repository.find_by_name(&news_paper_name).unwrap();
        let _result = news_paper.submit(article);

        // When
        news_paper_repository.update(&news_paper);

        // Then
        let option = news_paper_repository.find_by_name(&news_paper_name);
        assert_eq!(true, option.is_some());
        assert_eq!(news_paper.get_name(), &news_paper_name);
        assert_eq!(news_paper.get_articles().len(),1);
        let article = news_paper.get_articles().get(0).unwrap();
        assert_eq!(article.get_title(), &"Voldemort a été vaincu".to_string());
        assert_eq!(article.get_body(), &"Harry Potter a vaincu le seigneur des ténèbres !!".to_string());
        assert_eq!(article.is_published(), false);
    }

    fn it_should_update_article_when_it_has_been_published(connection: Rc<PgConnection>) {
        // Given
        let news_paper_name = "LaGazetteDuSorcier".to_string();
        let article = Article::new("Voldemort a été vaincu".to_string(), "Harry Potter a vaincu le seigneur des ténèbres !!".to_string());
        let news_paper_repository = NewsPaperRepositoryAdapter::new(connection);
        let mut news_paper = news_paper_repository.find_by_name(&news_paper_name).unwrap();
        let _result = news_paper.publish_article(article.get_title().clone());

        // When
        news_paper_repository.update(&news_paper);

        // Then
        let option = news_paper_repository.find_by_name(&news_paper_name);
        assert_eq!(true, option.is_some());
        assert_eq!(news_paper.get_name(), &news_paper_name);
        assert_eq!(news_paper.get_articles().len(),1);
        let article = news_paper.get_articles().get(0).unwrap();
        assert_eq!(article.get_title(), &"Voldemort a été vaincu".to_string());
        assert_eq!(article.get_body(), &"Harry Potter a vaincu le seigneur des ténèbres !!".to_string());
        assert_eq!(article.is_published(), true);
    }

    fn it_should_find_all_news_papers_with_all_articles(connection: Rc<PgConnection>) {
        // Given
        let repository_query = NewsPaperQueryRepositoryAdapter::new(connection);

        // When
        let news_papers = repository_query.find_all_even_with_unpublished_article();

        // Then
        assert_eq!(news_papers.len(), 2);
        let news_paper_1 = news_papers.get(0).unwrap();
        let news_paper_2 = news_papers.get(1).unwrap();
        assert_eq!(news_paper_1.articles.len(), 1);
        assert_eq!(news_paper_2.articles.len(), 1);
    }

    fn it_should_find_all_news_papers_with_published_articles(connection: Rc<PgConnection>) {
        // Given
        let repository_query = NewsPaperQueryRepositoryAdapter::new(connection);

        // When
        let news_papers = repository_query.find_all();

        // Then
        assert_eq!(news_papers.len(), 2);
        let news_paper_1 = news_papers.get(0).unwrap();
        let news_paper_2 = news_papers.get(1).unwrap();
        assert_eq!(news_paper_1.articles.len(), 1);
        assert_eq!(news_paper_2.articles.len(), 0);
    }

}