package org.tby.fourdk.security.sample.spring.usecases.commands;

import org.springframework.stereotype.Service;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.security.Permission;
import org.tby.fourdk.security.command.SecuredCommandHandler;
import org.tby.fourdk.security.sample.spring.domain.ParkingNotFoundException;
import org.tby.fourdk.security.sample.spring.domain.ParkingRepository;

import java.util.List;

import static org.tby.fourdk.security.sample.spring.domain.Permissions.PARKING_CONTROL;

@Service
public class ParkACarCommandHandler implements SecuredCommandHandler<ParkACarCommand> {

    private ParkingRepository parkingRepository;

    public ParkACarCommandHandler(ParkingRepository parkingRepository) {
        this.parkingRepository = parkingRepository;
    }

    @Override
    public List<Event> handle(ParkACarCommand command) {
        var parkingOpt = this.parkingRepository.findById(command.parkingId);
        if (parkingOpt.isPresent()) {
            var parking = parkingOpt.get();
            parking.parkACar(command.carId);
            this.parkingRepository.save(parking);
            return parking.getGeneratedEvents();
        } else {
            throw new ParkingNotFoundException("Parking with id [" + command.parkingId + "] not found");
        }
    }

    @Override
    public Class<ParkACarCommand> getCommandType() {
        return ParkACarCommand.class;
    }

    @Override
    public Permission getAssociatedPermission() {
        return PARKING_CONTROL;
    }
}
