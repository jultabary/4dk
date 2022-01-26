# Sample
Integration of 4dk in a Actix and Diesel context.<br>

## basic-4dk-sample-integrated-with-actix

### Diesel
<a href="https://diesel.rs/">Diesel</a> is a famous Rust ORM. 

### Actix
<a href="https://actix.rs/">Actix</a> is a famous Rust web framework.

## Presentation

### main.rs
`Context` class is the parent commandBus and QueryBus. (so the entry point of command and query) <br/>
`Context` factory `new` makes all the dependency injection needed for the project. <br/>
Only context is injected in the rocket container (manage).

### infrastructure/api/routes.rs
Only this file contains Actix code with `main.rs` <br />
Exposed routes:
- `get_all_foo` GET /foo
- `post_foo` POST /foo

### infrastructure/database/*.rs
Only this file contains Diesel <br />
It contains all code relative to the database <br />

## Start the sample
### start for dev
Please enter the following commands:
- `make install-diesel-cli` : install diesel cli (optional) 
- `make start-db` : start the postgres database 
- `make init-db` : run diesel migration
- `make start` : start the server 

### start for prod
Please enter the following commands:
- `make install-diesel-cli` : install diesel cli (optional) 
- `make init-db` : run diesel migration
- `make start-db` : start the postgres database
- `make release` : build for prod the app
- `make start-release` : start the server


