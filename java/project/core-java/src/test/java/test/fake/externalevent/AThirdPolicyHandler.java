package test.fake.externalevent;

import org.tby.fourdk.core.command.Command;

import java.util.Arrays;
import java.util.List;

public class AThirdPolicyHandler extends PolicyHandlerVerifiable<AnExternalEvent> {

    private Command command;

    public AThirdPolicyHandler(Command command) {
        this.command = command;
    }

    @Override
    public List<Command> handle(AnExternalEvent event) {
        super.handle(event);
        return Arrays.asList(command);
    }

    @Override
    public Class<AnExternalEvent> getEventType() {
        return AnExternalEvent.class;
    }
}
