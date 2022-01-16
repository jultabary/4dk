package org.tby.fourdk.core.sample.spring;

import org.springframework.stereotype.Component;
import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.command.CommandHandler;
import org.tby.fourdk.core.command.bus.CommandBus;
import org.tby.fourdk.core.command.bus.CommandDispatcher;
import org.tby.fourdk.core.command.bus.CommandLoggingMiddleware;
import org.tby.fourdk.core.command.bus.EventsProducedByCommandBusDispatcher;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.event.EventHandler;
import org.tby.fourdk.core.event.bus.EventDispatcher;
import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;
import org.tby.fourdk.core.query.bus.QueryBus;
import org.tby.fourdk.core.query.bus.QueryDispatcher;
import org.tby.fourdk.core.query.bus.QueryHandler;
import org.tby.fourdk.core.query.bus.QueryLoggingMiddleware;

import java.util.List;
import java.util.concurrent.Executors;

import static org.tby.fourdk.core.event.EventHandlerLoggerFactory.addLoggerToEventHandlers;

@Component
public class Application implements CommandBus, QueryBus {

    private CommandBus commandBus;

    private QueryBus queryBus;

    public Application(List<EventHandler> eventHandlers,
                       List<CommandHandler> commandHandlers,
                       List<QueryHandler> queryHandlers) {
        var commandDispatcher = new CommandDispatcher(commandHandlers);
        var eventDispatcher = new EventDispatcher(addLoggerToEventHandlers(eventHandlers));
        var loggingMiddleware = new CommandLoggingMiddleware(commandDispatcher);

        this.queryBus = new QueryLoggingMiddleware(new QueryDispatcher(queryHandlers));
        this.commandBus = new EventsProducedByCommandBusDispatcher(eventDispatcher, loggingMiddleware, Executors.newSingleThreadExecutor());
    }


    @Override
    public List<Event> dispatch(Command command) {
        return this.commandBus.dispatch(command);
    }

    @Override
    public Response dispatch(Query query) {
        return this.queryBus.dispatch(query);
    }
}
