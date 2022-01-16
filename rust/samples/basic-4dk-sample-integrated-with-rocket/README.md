# Sample
Integration of 4dk in a Rocket and Diesel context.<br>

## basic-4dk-sample-integrated-with-rocket

### Diesel
<a href="https://diesel.rs/">Diesel</a> is a famous Rust ORM. 

### Rocket
<a href="https://rocket.rs/">Rocket</a> is a famous Rust web framework.

## Start with the sample
Please enter the following commands:
- `make install-diesel-cli` : install diesel cli 
- `make start-db` : start the postgres database 
- `make start` : start the server 

`Context` class is the parent commandBus and QueryBus. (so the entry point of command and query) <br/>
`Context` factory `new` makes all the dependency injection needed for the project. <br/>
Only context is injected in the rocket container (manage).

