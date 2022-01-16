package test.fake.command;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.security.command.SecuredCommand;
import org.tby.fourdk.security.command.SecuredCommandHandler;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public abstract class VerifiableSecuredCommandHandler<C extends SecuredCommand> implements SecuredCommandHandler<C> {

    private List<Command> handledCommands;

    public VerifiableSecuredCommandHandler() {
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
