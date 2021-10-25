package org.tby.fourdk.security.sample.spring.domain;

import java.util.UUID;

public record ScreenId(UUID id) {

    @Override
    public String toString() {
        return "ScreenId{" +
                "id=" + id +
                '}';
    }
}
