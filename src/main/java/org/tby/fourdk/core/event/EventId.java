package org.tby.fourdk.core.event;

import org.tby.fourdk.core.command.CommandId;

import java.util.Objects;
import java.util.UUID;

public record EventId(UUID id) {

    public static EventId create() {
        return new EventId(UUID.randomUUID());
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        EventId eventId = (EventId) o;
        return Objects.equals(id, eventId.id);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }

    @Override
    public String toString() {
        return "EventId{" +
                "id=" + id +
                '}';
    }
}
