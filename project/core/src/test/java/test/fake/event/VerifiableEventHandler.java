package test.fake.event;

import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.event.EventHandler;

import java.util.ArrayList;
import java.util.List;

public abstract class VerifiableEventHandler<E extends Event> implements EventHandler<E> {

    private List<Event> handledEvents;

    public VerifiableEventHandler() {
        this.handledEvents = new ArrayList<>();
    }

    @Override
    public void handle(Event event) {
        this.handledEvents.add(event);
    }

    public boolean hasHandled(Event event) {
        return this.handledEvents.contains(event);
    }
}

