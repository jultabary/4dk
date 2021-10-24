package org.tby.fourdk.core.sample.spring.usecases.queries;

import org.tby.fourdk.core.query.Response;
import org.tby.fourdk.core.sample.spring.domain.CarId;

import java.util.List;

public class ParkedCarsResponse implements Response {

    public final List<CarId> parkedCars;

    public ParkedCarsResponse(List<CarId> parkedCars) {
        this.parkedCars = parkedCars;
    }

    @Override
    public String toString() {
        return "ParkedCarsResponse{" +
                "parkedCars=" + parkedCars +
                '}';
    }
}
