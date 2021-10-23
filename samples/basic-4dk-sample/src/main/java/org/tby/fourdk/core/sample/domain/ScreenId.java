package org.tby.fourdk.core.sample.domain;

import java.util.UUID;

public record ScreenId(UUID id) {

    @Override
    public String toString() {
        return "ScreenId{" +
                "id=" + id +
                '}';
    }
}
