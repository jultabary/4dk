package org.tby.fourdk.core.sample.usecases.commands;

import org.tby.fourdk.core.command.CommandHandler;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.sample.domain.ParkingNotFoundException;
import org.tby.fourdk.core.sample.domain.ParkingRepository;

import java.util.List;

public class ParkACarCommandHandler implements CommandHandler<ParkACarCommand> {

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
}
