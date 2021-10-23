package org.tby.fourdk.core.command.bus;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.event.Event;

import java.util.List;

public class CommandLoggingMiddleware implements CommandBus {

    private static final Logger LOGGER = LoggerFactory.getLogger(CommandLoggingMiddleware.class);

    private CommandBus nextMiddleware;

    public CommandLoggingMiddleware(CommandBus commandBus) {
        this.nextMiddleware = commandBus;
    }

    @Override
    public List<Event> dispatch(Command command) {
        LOGGER.info("Handling command [{}]", command);
        var begin =  System.currentTimeMillis();
        var type = command.getClass().getSimpleName();
        List<Event> events;
        try {
            events = this.nextMiddleware.dispatch(command);
        } catch (RuntimeException e) {
            LOGGER.info("An error occurred while handling command [{}], exception: {}", command, e.getClass().getSimpleName());
            LOGGER.debug("An error occurred while handling command [{}], exception: {}", command, e);
            throw e;
        }
        var end = System.currentTimeMillis();
        var timeToProcessCommand = end - begin;
        LOGGER.info("Command of type [{}] [{}] has been handled in [{}] ms and has produced the following events [{}] [{}]",
                type, command, timeToProcessCommand, events.size(), events);
        return events;
    }


}
