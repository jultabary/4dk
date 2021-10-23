package org.tby.fourdk.core.command.bus;

public class UnhandledCommandException extends RuntimeException {

    public UnhandledCommandException(String message) {
        super(message);
    }
}
