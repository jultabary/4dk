package org.tby.fourdk.security.sample.spring.usecases.commands;

import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.command.SecuredCommand;
import org.tby.fourdk.security.sample.spring.domain.CarId;
import org.tby.fourdk.security.sample.spring.domain.ParkingId;
import org.tby.fourdk.security.userinfo.UserId;

import java.util.List;

public class ParkACarCommand extends SecuredCommand {

    public final ParkingId parkingId;

    public final CarId carId;

    public ParkACarCommand(ParkingId parkingId, CarId carId, List<Role> roles, UserId userId) {
        super(roles, userId);
        this.parkingId = parkingId;
        this.carId = carId;
    }

    @Override
    public String toString() {
        return "ParkACarCommand{" +
                "id=" + id +
                ", userRoles=" + userRoles +
                ", userId=" + userId +
                ", parkingId=" + parkingId +
                ", carId=" + carId +
                '}';
    }
}
