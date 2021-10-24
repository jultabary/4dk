package org.tby.fourdk.core.sample.spring.infrastructure.persistence;

import org.springframework.stereotype.Component;
import org.tby.fourdk.core.sample.spring.domain.Parking;
import org.tby.fourdk.core.sample.spring.domain.ParkingId;
import org.tby.fourdk.core.sample.spring.domain.ParkingRepository;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

@Component
public class ParkingInMemoryRepository implements ParkingRepository {

    private final List<Parking> registeredParkings;

    public ParkingInMemoryRepository() {
        this.registeredParkings = new ArrayList<Parking>();
    }

    public void add(Parking parking) {
        this.registeredParkings.add(parking);
    }

    @Override
    public Optional<Parking> findById(ParkingId id) {
        return this.registeredParkings.stream().filter(p -> p.getId().equals(id)).findFirst();
    }

    @Override
    public void save(Parking parking) {
        // Nothing to do
    }
}
