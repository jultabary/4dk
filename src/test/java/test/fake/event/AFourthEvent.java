package test.fake.event;

import org.tby.fourdk.core.command.CommandId;
import org.tby.fourdk.core.event.Event;

import java.time.ZonedDateTime;

public class AFourthEvent extends Event {
    public AFourthEvent(CommandId parentCommandId, ZonedDateTime dateTime) {
        super(parentCommandId, dateTime);
    }
}
