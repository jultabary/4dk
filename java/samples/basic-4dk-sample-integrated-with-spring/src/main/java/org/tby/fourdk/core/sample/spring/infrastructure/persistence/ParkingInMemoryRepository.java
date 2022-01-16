package org.tby.fourdk.core.sample.spring.infrastructure.persistence;

import org.springframework.stereotype.Component;
import org.tby.fourdk.core.sample.spring.domain.*;

import java.util.*;

@Component
public class ParkingInMemoryRepository implements ParkingRepository, ParkingReadRepository {

    private List<Parking> registeredParkings;

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

    @Override
    public List<CarId> findByParkedCar(ParkingId id) {
        Set<CarId> cars = new LinkedHashSet<CarId>();
        for (Parking parking: this.registeredParkings) {
            cars.addAll(parking.getParkedCars());
        }
        return cars.stream().toList();
    }

    public void reset() {
        this.registeredParkings = new ArrayList<>();
    }
}
