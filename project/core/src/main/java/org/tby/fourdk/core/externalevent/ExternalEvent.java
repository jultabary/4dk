package org.tby.fourdk.core.externalevent;

import org.tby.fourdk.core.event.Event;

import java.time.ZonedDateTime;

public class ExternalEvent extends Event {
    public ExternalEvent(ZonedDateTime dateTime) {
        super(dateTime);
    }

}
