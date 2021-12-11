package org.tby.fourdk.core.command.bus;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.event.EventRepository;

import java.util.List;

public class EventsProducedByCommandBusPersistenceMiddleware implements CommandBus{

    private CommandBus nextMiddleware;

    private EventRepository eventRepository;

    public EventsProducedByCommandBusPersistenceMiddleware(CommandBus nextMiddleware, EventRepository eventRepository) {
        this.nextMiddleware = nextMiddleware;
        this.eventRepository = eventRepository;
    }

    @Override
    public List<Event> dispatch(Command command) {
        var events = this.nextMiddleware.dispatch(command);
        try {
            for (Event event : events) {
                this.eventRepository.save(event);
            }
        } catch (Exception e) {
            throw new EventPersistenceException("Error while persisting event.", e);
        }
        return events;
    }
}
