package org.tby.fourdk.core.sample.usecases.events;

import org.tby.fourdk.core.command.CommandId;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.sample.domain.CarId;
import org.tby.fourdk.core.sample.domain.ParkingId;

import java.time.ZonedDateTime;

public class ANewCarIsParkedEvent extends Event {

    public final ParkingId parkingId;
    public final CarId carId;

    public ANewCarIsParkedEvent(ZonedDateTime dateTime, ParkingId parkingId, CarId carId) {
        super(dateTime);
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
