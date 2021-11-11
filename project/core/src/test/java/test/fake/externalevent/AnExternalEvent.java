package test.fake.externalevent;

import org.tby.fourdk.core.externalevent.ExternalEvent;

import java.time.ZonedDateTime;

public class AnExternalEvent extends ExternalEvent {
    public AnExternalEvent(ZonedDateTime dateTime) {
        super(dateTime);
    }
}
