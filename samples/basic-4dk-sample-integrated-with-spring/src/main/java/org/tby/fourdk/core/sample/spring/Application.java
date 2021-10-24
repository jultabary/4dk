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

import java.util.List;
import java.util.concurrent.Executors;

import static org.tby.fourdk.core.event.EventHandlerLoggerFactory.addLoggerToEventHandlers;

@Component
public class Application implements CommandBus {

    private CommandBus commandBus;

    public Application(List<EventHandler> eventHandlers,
                       List<CommandHandler> commandHandlers) {
        var commandDispatcher = new CommandDispatcher(commandHandlers);
        var eventDispatcher = new EventDispatcher(addLoggerToEventHandlers(eventHandlers));
        var loggingMiddleware = new CommandLoggingMiddleware(commandDispatcher);

        this.commandBus = new EventsProducedByCommandBusDispatcher(eventDispatcher, loggingMiddleware, Executors.newSingleThreadExecutor());
    }


    @Override
    public List<Event> dispatch(Command command) {
        return this.commandBus.dispatch(command);
    }
}
