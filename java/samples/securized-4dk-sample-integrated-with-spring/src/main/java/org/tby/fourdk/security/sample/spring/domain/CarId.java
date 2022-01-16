package org.tby.fourdk.security.sample.spring.domain;

import java.util.Objects;
import java.util.UUID;

public record CarId(UUID id) {

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        CarId carId = (CarId) o;
        return Objects.equals(id, carId.id);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }
}
