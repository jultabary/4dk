package org.tby.fourdk.core.event.bus;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.event.EventHandler;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class EventDispatcher implements EventBus {

    private static final Logger LOGGER = LoggerFactory.getLogger(EventDispatcher.class);

    private Map<Class, List<EventHandler>> eventHandlers;

    public EventDispatcher(List<EventHandler> eventHandlers) {
        this.eventHandlers = new HashMap<>();
        for (EventHandler eventHandler : eventHandlers) {
            registerEventHandler(eventHandler);
        }
    }

    @Override
    public void dispatch(Event event) {
        if (isEventHandled(event)) {
            var eventHandlers = this.eventHandlers.get(event.getClass());
            eventHandlers.parallelStream().forEach(eventHandler -> {
                eventHandler.handle(event);
            });
        }
    }

    private boolean isEventHandled(Event event) {
        return this.eventHandlers.containsKey(event.getClass());
    }

    private void registerEventHandler(EventHandler eventHandler) {
        if (!this.eventHandlers.containsKey(eventHandler.getEventType())) {
            var handlerType = eventHandler.getName();
            var eventType = eventHandler.getEventType().getSimpleName();
            LOGGER.info("Register eventHandler / policy [{}] for event [{}]", handlerType, eventType);
            this.eventHandlers.put(eventHandler.getEventType(), new ArrayList<>());
        }
        if (isHandlerAlreadyRegistered(eventHandler)) {
            var type = eventHandler.getEventType();
            throw new EventAlreadyHandledException("EventHandler [" + type + "] is already registered");
        }
        this.eventHandlers.get(eventHandler.getEventType()).add(eventHandler);
    }

    private boolean isHandlerAlreadyRegistered(EventHandler eventHandler) {
        return this.eventHandlers.get(eventHandler.getEventType()).stream().anyMatch(eh -> eh.getName().equals(eventHandler.getName()));
    }

}
