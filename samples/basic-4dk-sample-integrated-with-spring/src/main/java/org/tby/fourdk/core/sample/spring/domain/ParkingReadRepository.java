package org.tby.fourdk.core.sample.spring.domain;

import java.util.List;

public interface ParkingReadRepository {
    List<CarId> findByParkedCar(ParkingId id);

}
