package org.tby.fourdk.security.sample.spring.domain;

import java.util.UUID;

public record GateId(UUID id) {

    @Override
    public String toString() {
        return "GateId{" +
                "id=" + id +
                '}';
    }
}
