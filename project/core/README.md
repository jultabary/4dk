# 4dk-core

The core project implements the primary concept of this framework. <br/>
It implements the entites : `Command`, `Event` and `Query`.

## Command

### Command
A `Command` is an object which triggers a usecase of type `CommandHandler`. It is an order sent to the system. The transaction will result a list of `Event`.<br/>
A `Command` is dispatched to the correct handler trough a `CommandBus`.

### CommandBus
A `CommandBus` valids the following interface `List<Event> dispatch(Command command)`.<br/>
This project proposes the following commandbus:
* `CommandDispatcher`: its constructor takes in entry a list of `CommandHandler`. This bus dispatches `Command` to correct `CommandHandler`.
* `CommandLoggingMiddleware`: its constructor takes in entry a `CommandBus`. This bus loggs all `Command` before be handling and after with its produced list of `Event`. It forwards received `Command` to the next `CommandBus`.
* `EventsProducedByCommandBusPersistenceMiddleware`: its constructor takes in entry a `CommandBus` and an `EventRepository`. This bus is used to implement "Event sourcing". It forwards received `Command` to the next `CommandBus`, then it saves all received `Event`s. `EventRepository` is only an interface. There is no implementation in this project for this interface, you have to implement it.
* `EventsProducedByCommandBusDispatcher`: its constructor takes in entry an `EventBus`, a `CommandBus` and an `ExecutorService`. This bus makes the link between `CommandBus` and `EventBus`, it dispatches received `Event`s to the `EventBus`.  The `ExecutorService` is used to dispatch `Event` asynchronously.

<br /><br />
You can find in samples different bus composition.

## Event
An `Event` is an object returned by a `CommandHandler`. An `Event` is a result of a business transaction. It could trigger usecases of type `EventHandler` <br />
An `Event` is dispatched to its associates handlers through an `EventBus`.

### EventBus
An `EventBus` valids the following interface `void dispatch(Event event)`.<br/>
This project proposes the following eventbus:
* `EventDispatcher`: its constructor takes in entry a list of `EventHandler`. This bus dispatches `Event` to its correct `EventHandler`.

<br /><br />
You can find in samples different bus composition.

## Query
A `Query` is an object which triggers a usecase of type `QueryHandler`. It is a question asked to the system. A `Response` is returned from a `QueryHandler.<br/>

### QueryBus
An `EventBus` valids the following interface `Response dispatch(Query query)`.<br/>
This project proposes the following querybus:
* `QueryDispatcher`: its constructor takes in entry a list of `QueryHandler`. This bus dispatches `Query` to its correct `QueryHandler`.
* `QueryLoggingMiddleware`: its constructor takes in entry a `QueryBus`. This bus loggs all `Query` before and after be handling. It also loggs the returned `Response`. It forwards received `Query` to the next `QueryBus`.
