package org.tby.fourdk.security.sample.spring.usecases.events;


import org.tby.fourdk.security.sample.spring.domain.CarId;
import org.tby.fourdk.security.sample.spring.domain.ParkingId;

import java.time.ZonedDateTime;

public class ACarParkHasBeenRefusedEvent extends SampleEvent {

    public final ParkingId parkingId;

    public final CarId carId;

    public final String reason;

    public ACarParkHasBeenRefusedEvent(ZonedDateTime dateTime, ParkingId parkingId, CarId carId, String reason) {
        super(dateTime, false);
        this.parkingId = parkingId;
        this.carId = carId;
        this.reason = reason;
    }

    @Override
    public String toString() {
        return "ACarParkHasBeenRefusedEvent{" +
                "id=" + id +
                ", utcDateTime=" + utcDateTime +
                ", parkingId=" + parkingId +
                ", carId=" + carId +
                ", reason='" + reason + '\'' +
                '}';
    }
}
