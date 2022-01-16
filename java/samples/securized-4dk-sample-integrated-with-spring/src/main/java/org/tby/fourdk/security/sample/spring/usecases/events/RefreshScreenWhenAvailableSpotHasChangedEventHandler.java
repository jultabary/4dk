package org.tby.fourdk.security.sample.spring.usecases.events;

import org.springframework.stereotype.Service;
import org.tby.fourdk.core.event.EventHandler;
import org.tby.fourdk.security.sample.spring.domain.ParkingRepository;
import org.tby.fourdk.security.sample.spring.domain.ScreenRepository;

@Service
public class RefreshScreenWhenAvailableSpotHasChangedEventHandler implements EventHandler<ANewCarIsParkedEvent> {

    private final ScreenRepository screenRepository;

    private final ParkingRepository parkingRepository;

    public RefreshScreenWhenAvailableSpotHasChangedEventHandler(ScreenRepository screenRepository, ParkingRepository parkingRepository) {
        this.screenRepository = screenRepository;
        this.parkingRepository = parkingRepository;
    }

    @Override
    public void handle(ANewCarIsParkedEvent event) {
        var parkingOpt = this.parkingRepository.findById(event.parkingId);
        if (parkingOpt.isPresent()) {
            var parking = parkingOpt.get();
            this.screenRepository.refreshTheDisplay(parking.getScreenId(), parking.getAvailableSpots());
        }
    }

    @Override
    public Class<ANewCarIsParkedEvent> getEventType() {
        return ANewCarIsParkedEvent.class;
    }
}
