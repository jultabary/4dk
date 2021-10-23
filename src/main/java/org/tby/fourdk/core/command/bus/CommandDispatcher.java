package org.tby.fourdk.core.command.bus;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.command.CommandHandler;
import org.tby.fourdk.core.event.Event;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class CommandDispatcher implements CommandBus {
    private static final Logger LOGGER = LoggerFactory.getLogger(CommandDispatcher.class);

    protected Map<Class, CommandHandler> commandHandlers;

    public CommandDispatcher(List<CommandHandler> commandHandlers) {
        this.commandHandlers = new HashMap<>();
        registerCommandHandlers(commandHandlers);
    }


    @Override
    public List<Event> dispatch(Command command) {
        if (isCommandHandled(command.getClass())) {
            var commandHandler = this.commandHandlers.get(command.getClass());
            return commandHandler.handle(command);
        } else {
            throw new UnhandledCommandException("Command of type [" + command.getClass().getSimpleName() + "] has no associated handler.");
        }
    }

    private void registerCommandHandlers(List<CommandHandler> commandHandlers) {
        for (CommandHandler commandHandler : commandHandlers) {
            if (!isCommandHandled(commandHandler.getCommandType())) {
                var handlerType = commandHandler.getName();
                var commandType = commandHandler.getCommandType().getSimpleName();
                this.commandHandlers.put(commandHandler.getCommandType(), commandHandler);
                LOGGER.info("Register commandHandler [{}] for command [{}]", handlerType, commandType);
            } else {
                throw new CommandAlreadyHandledException("Command of type [" + commandHandler.getCommandType() + "] already handled");
            }
        }
    }

    private Boolean isCommandHandled(Class commandClass) {
        return this.commandHandlers.containsKey(commandClass);
    }

}
