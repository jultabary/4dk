package org.tby.fourdk.core.sample.spring.usecases.commands;

import org.springframework.stereotype.Service;
import org.tby.fourdk.core.command.CommandHandler;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.sample.spring.domain.ParkingRepository;
import org.tby.fourdk.core.sample.spring.domain.ParkingNotFoundException;

import java.util.List;

@Service
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
