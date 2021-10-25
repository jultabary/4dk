package org.tby.fourdk.security.sample.spring.domain;

import java.util.Optional;

public interface ParkingRepository {
    Optional<Parking> findById(ParkingId id);

    void save(Parking parking);
}
