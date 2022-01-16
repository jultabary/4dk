# 4dk-core

The core project implements the primary concept of this framework. <br/>
It implements the entites : `Command`, `Event` and `Query`.

## Command

### Command
A `Command` is an object which triggers a usecase of type `CommandHandler`. It is an order sent to the system. The transaction will result a list of `Event`.<br/>
A `Command` is dispatched to the correct handler trough a `CommandBus`.

### CommandBus
A `CommandBus` valids the following trait `fn dispatch(command: &Command) -> Vec<Box<dyn Event>>`.<br/>
This project proposes the following commandbus:
* `CommandDispatcher`: its new factory takes in entry a list of `CommandHandler`. This bus dispatches `Command` to correct `CommandHandler`.

<br /><br />
You can find in samples different bus composition.

## Event
An `Event` is an object returned by a `CommandHandler`. An `Event` is a result of a business transaction. It could trigger usecases of type `EventHandler` <br />
An `Event` is dispatched to its associates handlers through an `EventBus`.

## Query
A `Query` is an object which triggers a usecase of type `QueryHandler`. It is a question asked to the system. A `Response` is returned from a `QueryHandler.<br/>

### QueryBus
An `EventBus` valids the following interface `fn dispatch(query: &Query) -> Vec<Box<dyn Response>>`.<br/>
This project proposes the following querybus:
* `QueryDispatcher`: its new factory takes in entry a list of `QueryHandler`. This bus dispatches `Query` to its correct `QueryHandler`.
