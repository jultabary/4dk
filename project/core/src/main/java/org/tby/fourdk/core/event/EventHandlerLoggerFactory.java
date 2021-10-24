package org.tby.fourdk.core.event;

import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.stream.Collectors;

public class EventHandlerLoggerFactory {

    public static List<EventHandler> addLoggerToEventHandlers(List<EventHandler> eventHandlers) {
        return eventHandlers.stream().map(EventHandlerLogger::new).collect(Collectors.toList());
    }
}

class EventHandlerLogger<T extends Event> implements EventHandler<T> {

    private EventHandler<T> eventHandler;

    EventHandlerLogger(EventHandler eventHandler) {
        this.eventHandler = eventHandler;
    }


    @Override
    public void handle(T event) {
        var begin = System.currentTimeMillis();

        var logger = LoggerFactory.getLogger(EventHandlerLogger.class);
        var eventType = event.getClass().getSimpleName();
        var eventHandlerType = eventHandler.getName();
        logger.info("Handling event of type [{}] with handler of type [{}]. Handled event is [{}]", eventType, eventHandlerType, event);
        try {
            eventHandler.handle(event);
        } catch (RuntimeException exception) {
            logger.error(
                    "Something went wrong while handling event of type [{}] with handler of type [{}]. Handled event was [{}]",
                    eventType,
                    eventHandlerType,
                    event,
                    exception
            );
            throw exception;
        }
        var end = System.currentTimeMillis();
        var timeToProcessEvent = end - begin;

        logger.info(
                "Event of type [{}] was handled successfully in [{}] milliseconds by handler of type [{}]. Handled event was [{}]",
                eventType,
                timeToProcessEvent,
                eventHandlerType,
                event
        );
    }

    @Override
    public Class<T> getEventType() {
        return this.eventHandler.getEventType();
    }

    @Override
    public String getName() {
        return this.eventHandler.getName();
    }
}

