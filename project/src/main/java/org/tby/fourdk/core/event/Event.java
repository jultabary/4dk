package org.tby.fourdk.core.event;

import org.tby.fourdk.core.command.CommandId;

import java.time.ZonedDateTime;

import static java.time.ZoneOffset.UTC;

public abstract class Event {

    public final EventId id;

    public final CommandId parentCommandId;

    public final ZonedDateTime utcDateTime;

    public Event(CommandId parentCommandId, ZonedDateTime dateTime) {
        this.id = EventId.create();
        this.parentCommandId = parentCommandId;
        this.utcDateTime = dateTime.withZoneSameInstant(UTC);
    }

}
