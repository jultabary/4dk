package org.tby.fourdk.core.command;

import org.tby.fourdk.core.event.Event;

import java.util.List;

public interface CommandHandler<C extends Command> {

    List<Event> handle(C command);

    Class<C> getCommandType();

    default String getName() {
        return this.getClass().getSimpleName();
    }
}