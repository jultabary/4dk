package org.tby.fourdk.core.command.bus;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.event.bus.EventBus;

import java.util.List;
import java.util.concurrent.ExecutorService;

public class EventsProducedByCommandBusDispatcher implements CommandBus {

    private static final Logger LOGGER = LoggerFactory.getLogger(EventsProducedByCommandBusDispatcher.class);

    private EventBus eventBus;

    private CommandBus nextMiddleware;

    private final ExecutorService executor;

    public EventsProducedByCommandBusDispatcher(EventBus eventBus, CommandBus commandBus, ExecutorService executor) {
        this.eventBus = eventBus;
        this.nextMiddleware = commandBus;
        this.executor = executor;
    }

    @Override
    public List<Event> dispatch(Command command) {
        var events = this.nextMiddleware.dispatch(command);
        for (Event event : events) {
            Runnable runnableTask = () -> {
                this.dispatchOneEvent(event);
            };
            executor.execute(runnableTask);
        }
        return events;
    }

    private void dispatchOneEvent(Event event) {
        try {
            this.eventBus.dispatch(event);
        } catch (RuntimeException exception) {
            var eventType = event.getClass().getSimpleName();
            LOGGER.error("Something went wrong while handling event of type [{}] [{}]", eventType, event, exception);
            throw exception;
        }
    }

}
