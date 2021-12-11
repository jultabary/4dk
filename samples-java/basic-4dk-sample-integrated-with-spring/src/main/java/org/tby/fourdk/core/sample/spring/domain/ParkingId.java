package org.tby.fourdk.core.sample.spring.domain;

import java.util.Objects;
import java.util.UUID;

public record ParkingId(UUID id) {

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        ParkingId parkingId = (ParkingId) o;
        return Objects.equals(id, parkingId.id);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }

    @Override
    public String toString() {
        return "ParkingId{" +
                "id=" + id +
                '}';
    }
}
