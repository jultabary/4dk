package org.tby.fourdk.core.sample.usecases.commands;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.sample.domain.CarId;
import org.tby.fourdk.core.sample.domain.ParkingId;

public class ParkACarCommand extends Command {

    public final ParkingId parkingId;

    public final CarId carId;

    public ParkACarCommand(ParkingId parkingId, CarId carId) {
        this.parkingId = parkingId;
        this.carId = carId;
    }

    @Override
    public String toString() {
        return "ParkACar{" +
                "id=" + id +
                ", parkingId=" + parkingId +
                ", carId=" + carId +
                '}';
    }
}
