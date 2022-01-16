package org.tby.fourdk.core.sample.spring.usecases.events;

import org.tby.fourdk.core.sample.spring.domain.CarId;
import org.tby.fourdk.core.sample.spring.domain.ParkingId;

import java.time.ZonedDateTime;

public class ANewCarIsParkedEvent extends SampleEvent {

    public final ParkingId parkingId;
    public final CarId carId;

    public ANewCarIsParkedEvent(ZonedDateTime dateTime, ParkingId parkingId, CarId carId) {
        super(dateTime, true);
        this.parkingId = parkingId;
        this.carId = carId;
    }

    @Override
    public String toString() {
        return "ANewCarIsParkedEvent{" +
                "id=" + id +
                ", utcDateTime=" + utcDateTime +
                ", parkingId=" + parkingId +
                ", carId=" + carId +
                '}';
    }
}
