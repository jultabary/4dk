package org.tby.fourdk.core.sample;

import org.junit.jupiter.api.Test;
import org.tby.fourdk.core.sample.infrastructure.AFakeGateRepositoryImpl;
import org.tby.fourdk.core.sample.infrastructure.AFakeScreenRepositoryImpl;
import org.tby.fourdk.core.sample.infrastructure.ParkingInMemoryRepository;
import org.tby.fourdk.core.sample.usecases.commands.ParkACarCommand;
import org.tby.fourdk.core.sample.usecases.events.ACarParkHasBeenRefusedEvent;
import org.tby.fourdk.core.sample.usecases.events.ANewCarIsParkedEvent;

import static org.assertj.core.api.Assertions.assertThat;
import static test.referential.TestReferential.*;

class ApplicationTest {

    @Test
    public void it_should_dispatch_the_command_and_return_successful_event() {
        // Given
        var parking = A_PARKING;
        var car = A_CAR;

        var parkingRepo = new ParkingInMemoryRepository();
        var screenRepo = new AFakeScreenRepositoryImpl();
        var gateRepo = new AFakeGateRepositoryImpl();

        parkingRepo.add(parking);

        var application = new Application(parkingRepo, screenRepo, gateRepo);
        var command = new ParkACarCommand(parking.getId(), car);

        // When
        var events = application.dispatch(command);

        // Then
        assertThat(events).hasSize(1);
        var event = events.get(0);
        assertThat(event).isInstanceOf(ANewCarIsParkedEvent.class);
        assertThat(((ANewCarIsParkedEvent) event).carId).isEqualTo(A_CAR);
        assertThat(((ANewCarIsParkedEvent) event).parkingId).isEqualTo(A_PARKING.getId());
        assertThat(screenRepo.getHasBeenCalled()).isTrue();
        assertThat(gateRepo.getHasBeenCalled()).isTrue();
    }

    @Test
    public void it_should_dispatch_the_command_and_return_a_refuse_event() {
        // Given
        var parking = A_SMALL_PARKING;
        var car = A_CAR;

        var parkingRepo = new ParkingInMemoryRepository();
        var screenRepo = new AFakeScreenRepositoryImpl();
        var gateRepo = new AFakeGateRepositoryImpl();

        parkingRepo.add(parking);

        var application = new Application(parkingRepo, screenRepo, gateRepo);
        var command = new ParkACarCommand(parking.getId(), car);

        // When
        var events = application.dispatch(command);

        // Then
        assertThat(events).hasSize(1);
        var event = events.get(0);
        assertThat(event).isInstanceOf(ACarParkHasBeenRefusedEvent.class);
    }

}