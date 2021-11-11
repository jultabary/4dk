package test.fake.externalevent;

import org.tby.fourdk.core.externalevent.ExternalEvent;

import java.time.ZonedDateTime;

public class AnotherExternalEvent extends ExternalEvent {
    public AnotherExternalEvent(ZonedDateTime dateTime) {
        super(dateTime);
    }
}
