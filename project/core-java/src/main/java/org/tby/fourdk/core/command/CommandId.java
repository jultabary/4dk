package org.tby.fourdk.core.command;

import java.util.Objects;
import java.util.UUID;

public record CommandId(UUID id) {

    public static CommandId create() {
        return new CommandId(UUID.randomUUID());
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        CommandId commandId = (CommandId) o;
        return Objects.equals(id, commandId.id);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }

    @Override
    public String toString() {
        return "CommandId{" +
                "id=" + id +
                '}';
    }
}
