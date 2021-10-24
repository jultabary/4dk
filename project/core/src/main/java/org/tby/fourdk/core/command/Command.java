package org.tby.fourdk.core.command;

public abstract class Command {

    public final CommandId id;

    public Command() {
        this.id = CommandId.create();
    }

}
