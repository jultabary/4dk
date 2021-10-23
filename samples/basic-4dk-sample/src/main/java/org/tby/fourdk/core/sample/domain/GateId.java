package org.tby.fourdk.core.sample.domain;

import java.util.UUID;

public record GateId(UUID id) {

    @Override
    public String toString() {
        return "GateId{" +
                "id=" + id +
                '}';
    }
}
