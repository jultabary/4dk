package org.tby.fourdk.core.externalevent;

import org.tby.fourdk.core.command.Command;

import java.util.List;

public interface PolicyHandler<E extends ExternalEvent> {

    public List<Command> handle(E event);

    public Class<E> getEventType();

}
