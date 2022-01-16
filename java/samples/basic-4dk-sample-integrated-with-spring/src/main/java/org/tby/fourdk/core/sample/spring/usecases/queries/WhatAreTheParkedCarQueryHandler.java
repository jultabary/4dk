package org.tby.fourdk.core.sample.spring.usecases.queries;

import org.springframework.stereotype.Component;
import org.tby.fourdk.core.query.Response;
import org.tby.fourdk.core.query.bus.QueryHandler;
import org.tby.fourdk.core.sample.spring.domain.CarId;
import org.tby.fourdk.core.sample.spring.domain.ParkingReadRepository;

import java.util.List;

@Component
public class WhatAreTheParkedCarQueryHandler implements QueryHandler<WhatAreTheParkedCarQuery> {

    private ParkingReadRepository parkingReadRepository;

    public WhatAreTheParkedCarQueryHandler(ParkingReadRepository parkingReadRepository) {
        this.parkingReadRepository = parkingReadRepository;
    }

    @Override
    public Response ask(WhatAreTheParkedCarQuery query) {
        List<CarId> cars = this.parkingReadRepository.findByParkedCar(query.parkingId);
        return new ParkedCarsResponse(cars);
    }

    @Override
    public Class<WhatAreTheParkedCarQuery> getQueryType() {
        return WhatAreTheParkedCarQuery.class;
    }
}
