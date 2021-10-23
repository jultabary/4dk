package org.tby.fourdk.core.sample.domain;

import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.sample.usecases.events.ACarParkHasBeenRefusedEvent;
import org.tby.fourdk.core.sample.usecases.events.ANewCarIsParkedEvent;

import java.time.ZonedDateTime;
import java.util.ArrayList;
import java.util.List;

public class Parking {

    private final ParkingId id;

    private final ScreenId screenId;

    private final GateId gateId;

    private final Integer totalSpots;

    private final List<CarId> parkedCars;

    private final List<Event> generatedEvents;

    public Parking(ParkingId id, ScreenId screenId, GateId gateId, Integer totalSpots, List<CarId> parkedCars) {
        this.id = id;
        this.screenId = screenId;
        this.gateId = gateId;
        this.totalSpots = totalSpots;
        this.parkedCars = parkedCars;
        this.generatedEvents = new ArrayList<Event>();
    }

    public void parkACar(CarId carId) {
        if (areThereAvailableSpots()) {
            parkedCars.add(carId);
            this.generatedEvents.add(new ANewCarIsParkedEvent(ZonedDateTime.now(), this.id, carId));
        } else {
            this.generatedEvents.add(new ACarParkHasBeenRefusedEvent(ZonedDateTime.now(), this.id, carId, "No available Spots"));
        }
    }

    public ParkingId getId() {
        return id;
    }

    public GateId getGateId() {
        return gateId;
    }

    public ScreenId getScreenId() {
        return screenId;
    }

    public Integer getAvailableSpots() {
        return this.totalSpots - this.parkedCars.size();
    }

    public List<Event> getGeneratedEvents() {
        return this.generatedEvents;
    }

    private Boolean areThereAvailableSpots() {
        return totalSpots - parkedCars.size() > 0;
    }
}
