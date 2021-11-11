package org.tby.fourdk.core.externalevent;


import org.tby.fourdk.core.command.Command;

import java.util.List;

public interface ExternalEventBus {
    List<Command> dispatch(ExternalEvent event);
}
