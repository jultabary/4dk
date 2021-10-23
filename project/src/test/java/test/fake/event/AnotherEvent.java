package test.fake.event;

import org.tby.fourdk.core.command.CommandId;
import org.tby.fourdk.core.event.Event;

import java.time.ZonedDateTime;

public class AnotherEvent extends Event {
    public AnotherEvent(CommandId parentCommandId, ZonedDateTime dateTime) {
        super(parentCommandId, dateTime);
    }
}
