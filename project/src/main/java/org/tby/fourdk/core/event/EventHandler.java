package org.tby.fourdk.core.event;

public interface EventHandler<E extends Event> {

    void handle(E event);

    Class<E> getEventType();

    default String getName() {
        return this.getClass().getSimpleName();
    }

}
