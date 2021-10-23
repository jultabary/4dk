package org.tby.fourdk.core.command.bus;

public class EventPersistenceException extends RuntimeException {

    public EventPersistenceException(String message, Exception e) {
        super(message, e);
    }
}
