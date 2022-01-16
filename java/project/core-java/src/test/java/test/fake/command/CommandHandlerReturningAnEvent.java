package test.fake.command;

import org.tby.fourdk.core.event.Event;

import java.util.List;

public class CommandHandlerReturningAnEvent extends VerifiableCommandHandler<ACommand>{

    private List<Event> returnEvents;

    public CommandHandlerReturningAnEvent(List<Event> events) {
        this.returnEvents = events;
    }

    @Override
    public List<Event> handle(ACommand command) {
        super.handle(command);
        return this.returnEvents;
    }

    @Override
    public Class<ACommand> getCommandType() {
        return ACommand.class;
    }
}
