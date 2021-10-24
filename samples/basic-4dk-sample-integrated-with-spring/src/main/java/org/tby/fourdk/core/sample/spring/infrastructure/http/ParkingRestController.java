package org.tby.fourdk.core.sample.spring.infrastructure.http;

import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;
import org.tby.fourdk.core.command.bus.CommandBus;
import org.tby.fourdk.core.sample.spring.domain.CarId;
import org.tby.fourdk.core.sample.spring.domain.ParkingId;
import org.tby.fourdk.core.sample.spring.usecases.commands.ParkACarCommand;
import org.tby.fourdk.core.sample.spring.usecases.events.SampleEvent;

@RestController
public class ParkingRestController {

    private CommandBus commandBus;

    public ParkingRestController(CommandBus commandBus) {
        this.commandBus = commandBus;
    }

    @PostMapping(value = "/parkACar", consumes = MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<SampleEvent> parkACar(@RequestBody ParkACarRequest parkACarRequest) {
        var command = new ParkACarCommand(new ParkingId(parkACarRequest.parkingId()), new CarId(parkACarRequest.carId()));
        var events = this.commandBus.dispatch(command);
        // It is very bad to return the first event, it could have multiple events to consider.
        return createResponse((SampleEvent) events.get(0));
    }


    private ResponseEntity<SampleEvent> createResponse(SampleEvent event) {
        var httpStatus = event.isTransactionSucceed ? HttpStatus.OK: HttpStatus.CONFLICT;
        return new ResponseEntity<SampleEvent>(event, httpStatus);
    }

}
