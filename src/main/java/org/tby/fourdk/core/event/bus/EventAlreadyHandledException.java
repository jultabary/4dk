package org.tby.fourdk.core.event.bus;

public class EventAlreadyHandledException extends RuntimeException {

    public EventAlreadyHandledException(String message) {
        super(message);
    }
}
