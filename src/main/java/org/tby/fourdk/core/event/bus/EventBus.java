package org.tby.fourdk.core.event.bus;

import org.tby.fourdk.core.event.Event;

public interface EventBus {

    void dispatch(Event event);
}
