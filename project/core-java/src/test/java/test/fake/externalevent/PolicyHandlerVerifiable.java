package test.fake.externalevent;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.externalevent.ExternalEvent;
import org.tby.fourdk.core.externalevent.PolicyHandler;

import java.util.ArrayList;
import java.util.List;

public abstract class PolicyHandlerVerifiable<E extends ExternalEvent> implements PolicyHandler<E> {

    protected List<ExternalEvent> handledExternalEvents;

    public PolicyHandlerVerifiable() {
        this.handledExternalEvents = new ArrayList<>();
    }

    @Override
    public List<Command> handle(E event) {
        this.handledExternalEvents.add(event);
        return new ArrayList<>();
    }

    public boolean hasHandled(ExternalEvent externalEvent) {
        return this.handledExternalEvents.contains(externalEvent);
    }
}
