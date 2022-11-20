#[cfg(test)]
mod publish_article_if_validated_policy_handler_test {
    use dddk_core::dddk::external_event::policy_handler::PolicyHandler;
    use crate::usecases::policies::article_has_been_reviewed_event::ArticleHasBeenReviewedEvent;
    use crate::usecases::policies::publish_article_if_validated_policy_handler::PublishArticleIfValidatedPolicyHandler;

    #[test]
    fn it_should_not_return_command_when_article_is_not_validated() {
        // Given
        let policy = PublishArticleIfValidatedPolicyHandler {};
        let event = ArticleHasBeenReviewedEvent {
            news_paper_name: "ANewsPaper".to_string(),
            article_title: "ATitle".to_string(),
            is_validated: false
        };

        // When
        let commands = policy.handle(&event);

        // Then
        assert_eq!(0, commands.unwrap().len())
    }

    #[test]
    fn it_should_not_return_command_when_article_is_validated() {
        // Given
        let policy = PublishArticleIfValidatedPolicyHandler {};
        let event = ArticleHasBeenReviewedEvent {
            news_paper_name: "ANewsPaper".to_string(),
            article_title: "ATitle".to_string(),
            is_validated: true
        };

        // When
        let commands = policy.handle(&event);

        // Then
        assert_eq!(1, commands.unwrap().len())
    }

}