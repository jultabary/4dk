package org.tby.fourdk.core.sample.usecases.events;

import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.sample.domain.CarId;
import org.tby.fourdk.core.sample.domain.ParkingId;

import java.time.ZonedDateTime;

public class ACarParkHasBeenRefusedEvent extends Event {

    public final ParkingId parkingId;

    public final CarId carId;

    public final String reason;

    public ACarParkHasBeenRefusedEvent(ZonedDateTime dateTime, ParkingId parkingId, CarId carId, String reason) {
        super(dateTime);
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
