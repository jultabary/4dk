package org.tby.fourdk.core.sample.spring.usecases.queries;

import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.sample.spring.domain.ParkingId;

public class WhatAreTheParkedCarQuery implements Query {

    public final ParkingId parkingId;

    public WhatAreTheParkedCarQuery(ParkingId parkingId) {
        this.parkingId = parkingId;
    }

    @Override
    public String toString() {
        return "WhatAreTheParkedCarQuery{" +
                "parkingId=" + parkingId +
                '}';
    }
}
