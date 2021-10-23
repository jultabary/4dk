package org.tby.fourdk.core.command.bus;

public class CommandAlreadyHandledException extends RuntimeException {

    public CommandAlreadyHandledException(String message) {
        super(message);
    }
}
