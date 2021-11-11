package org.tby.fourdk.core.externalevent;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.command.bus.CommandBus;

import java.util.List;

public class CommandDispatcherAfterExternalEventBusMiddleware implements ExternalEventBus {

    private CommandBus commandBus;

    private ExternalEventBus nextMiddleware;

    public CommandDispatcherAfterExternalEventBusMiddleware(CommandBus commandBus, ExternalEventBus nextMiddleware) {
        this.commandBus = commandBus;
        this.nextMiddleware = nextMiddleware;
    }

    @Override
    public List<Command> dispatch(ExternalEvent event) {
        var commands = this.nextMiddleware.dispatch(event);
        commands.stream().forEach(command -> this.commandBus.dispatch(command));
        return commands;
    }
}
