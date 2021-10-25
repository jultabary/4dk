package org.tby.fourdk.security.sample.spring.domain;

import java.util.List;

public interface ParkingReadRepository {
    List<CarId> findByParkedCar(ParkingId id);

}
