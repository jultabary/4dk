package test.fake.event;

import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.event.EventRepository;

import java.util.ArrayList;
import java.util.List;

public class FakeEventRepository implements EventRepository {

    private List<Event> persistedEvents;

    public FakeEventRepository() {
        this.persistedEvents = new ArrayList<>();
    }

    @Override
    public void save(Event event) {
        if (event instanceof AnotherEvent) {
            // Simulate an issue with infrastructure
            throw new RuntimeException("There is an issue");
        }
        this.persistedEvents.add(event);
    }

    public Boolean isEventPersisted(Event event) {
        return this.persistedEvents.contains(event);
    }


}
