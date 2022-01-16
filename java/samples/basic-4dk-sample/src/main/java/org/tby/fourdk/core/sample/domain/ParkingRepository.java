package org.tby.fourdk.core.sample.domain;

import java.util.Optional;

public interface ParkingRepository {
    Optional<Parking> findById(ParkingId id);

    void save(Parking parking);
}
