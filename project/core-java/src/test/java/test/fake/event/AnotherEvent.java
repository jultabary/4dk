package test.fake.event;

import org.tby.fourdk.core.event.Event;

import java.time.ZonedDateTime;

public class AnotherEvent extends Event {
    public AnotherEvent(ZonedDateTime dateTime) {
        super(dateTime);
    }
}
