package test.fake.event;

import org.tby.fourdk.core.command.CommandId;
import org.tby.fourdk.core.event.Event;

import java.time.ZonedDateTime;

public class AnEvent extends Event {
    public AnEvent(CommandId parentCommandId, ZonedDateTime dateTime) {
        super(parentCommandId, dateTime);
    }
}
