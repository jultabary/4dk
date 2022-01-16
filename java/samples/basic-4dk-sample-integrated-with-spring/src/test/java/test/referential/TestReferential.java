package test.referential;

import org.tby.fourdk.core.sample.spring.domain.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.UUID;

public class TestReferential {

    public static final Parking A_PARKING = new Parking(
            new ParkingId(UUID.randomUUID()),
            new ScreenId(UUID.randomUUID()),
            new GateId(UUID.randomUUID()),
            30,
            new ArrayList<>()
    );

    public static final CarId A_CAR = new CarId(UUID.randomUUID());

    public static final CarId ANOTHER_CAR = new CarId(UUID.randomUUID());

    public static final Parking A_SMALL_PARKING = new Parking(
            new ParkingId(UUID.randomUUID()),
            new ScreenId(UUID.randomUUID()),
            new GateId(UUID.randomUUID()),
            1,
            Arrays.asList(ANOTHER_CAR)
    );

}
