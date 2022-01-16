package org.tby.fourdk.core.command.bus;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.event.Event;

import java.util.List;

public interface CommandBus {

    List<Event> dispatch(Command command);
}
