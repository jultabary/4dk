package org.tby.fourdk.core.externalevent;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.tby.fourdk.core.command.Command;

import java.util.List;

public class ExternalEventLoggingMiddleware implements ExternalEventBus {

    private static final Logger LOGGER = LoggerFactory.getLogger(ExternalEventLoggingMiddleware.class);

    private ExternalEventBus nextMiddleware;

    public ExternalEventLoggingMiddleware(ExternalEventBus nextMiddleware) {
        this.nextMiddleware = nextMiddleware;
    }

    @Override
    public List<Command> dispatch(ExternalEvent event) {
        var type = event.getClass().getSimpleName();
        LOGGER.info("Handling external event [{}] [{}]", type, event);
        var begin =  System.currentTimeMillis();
        List<Command> commands;
        try {
            commands = this.nextMiddleware.dispatch(event);
        } catch (RuntimeException e) {
            LOGGER.info("An error occurred while handling external event [{}]", event);
            throw e;
        }
        var end = System.currentTimeMillis();
        var timeToProcessCommand = end - begin;
        LOGGER.info("Event of type [{}] [{}] has been handled in [{}] ms and has produced the following commands [{}] [{}]",
            type, event, timeToProcessCommand, commands.size(), commands);
        return commands;
    }
}
