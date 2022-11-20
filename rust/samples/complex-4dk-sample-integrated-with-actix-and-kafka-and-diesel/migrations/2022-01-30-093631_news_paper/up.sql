CREATE TABLE news_papers
(
    name varchar(100) not null PRIMARY KEY
);

CREATE TABLE articles (
    title varchar(100) not null PRIMARY KEY,
    body varchar(100) not null,
    news_paper_name varchar(100) not null REFERENCES news_papers(name)
);
