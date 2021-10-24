package org.tby.fourdk.core.sample.spring.infrastructure.http;

import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.tby.fourdk.core.command.bus.CommandBus;
import org.tby.fourdk.core.query.bus.QueryBus;
import org.tby.fourdk.core.sample.spring.domain.CarId;
import org.tby.fourdk.core.sample.spring.domain.ParkingId;
import org.tby.fourdk.core.sample.spring.usecases.commands.ParkACarCommand;
import org.tby.fourdk.core.sample.spring.usecases.events.SampleEvent;
import org.tby.fourdk.core.sample.spring.usecases.queries.ParkedCarsResponse;
import org.tby.fourdk.core.sample.spring.usecases.queries.WhatAreTheParkedCarQuery;

import java.util.UUID;

@RestController
public class ParkingRestController {

    private CommandBus commandBus;

    private QueryBus queryBus;

    public ParkingRestController(CommandBus commandBus, QueryBus queryBus) {
        this.commandBus = commandBus;
        this.queryBus = queryBus;
    }

    @PostMapping(value = "/parkACar", consumes = MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<SampleEvent> parkACar(@RequestBody ParkACarRequest parkACarRequest) {
        var command = new ParkACarCommand(new ParkingId(parkACarRequest.parkingId()), new CarId(parkACarRequest.carId()));
        var events = this.commandBus.dispatch(command);
        // It is very bad to return the first event, it could have multiple events to consider.
        return createResponse((SampleEvent) events.get(0));
    }

    @GetMapping(value = "/parking/{parkingId}/parkedCar", produces = MediaType.APPLICATION_JSON_VALUE)
    public ParkedCarsResponse getParkedCar(@PathVariable("parkingId") UUID parkingId) {
        var query = new WhatAreTheParkedCarQuery(new ParkingId(parkingId));
        return (ParkedCarsResponse) this.queryBus.dispatch(query);

    }
    private ResponseEntity<SampleEvent> createResponse(SampleEvent event) {
        var httpStatus = event.isTransactionSucceed ? HttpStatus.OK: HttpStatus.CONFLICT;
        return new ResponseEntity<SampleEvent>(event, httpStatus);
    }

}
