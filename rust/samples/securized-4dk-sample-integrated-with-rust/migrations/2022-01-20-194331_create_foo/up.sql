CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE foo(
                    id UUID PRIMARY KEY,
                    title varchar(100) not null
);
