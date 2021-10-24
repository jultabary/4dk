package org.tby.fourdk.core.event;

import java.time.ZonedDateTime;

import static java.time.ZoneOffset.UTC;

public abstract class Event {

    public final EventId id;

    public final ZonedDateTime utcDateTime;

    public Event(ZonedDateTime dateTime) {
        this.id = EventId.create();
        this.utcDateTime = dateTime.withZoneSameInstant(UTC);
    }

}
