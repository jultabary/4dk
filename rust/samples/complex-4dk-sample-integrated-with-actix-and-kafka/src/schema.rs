table! {
    articles (title) {
        title -> Varchar,
        body -> Varchar,
        news_paper_name -> Varchar,
    }
}

table! {
    news_papers (name) {
        name -> Varchar,
    }
}

joinable!(articles -> news_papers (news_paper_name));

allow_tables_to_appear_in_same_query!(
    articles,
    news_papers,
);
