package test.fake.command;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.command.CommandHandler;
import org.tby.fourdk.core.event.Event;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public abstract class VerifiableCommandHandler<C extends Command> implements CommandHandler<C> {

    private List<Command> handledCommands;

    public VerifiableCommandHandler() {
        this.handledCommands = new ArrayList<>();
    }

    @Override
    public List<Event> handle(C command) {
        handledCommands.add(command);
        return Collections.emptyList();
    }

    public Boolean hasHandled(Command c) {
        return this.handledCommands.contains(c);
    }
}
