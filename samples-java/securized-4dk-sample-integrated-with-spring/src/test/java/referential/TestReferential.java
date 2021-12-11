package referential;

import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.sample.spring.domain.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.UUID;

public class TestReferential {

    public static String OPERATOR_TOKEN = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZW1haWwiOiJqb2huQGRvZS5uZXQiLCJyb2xlIjpbIk9QRVJBVE9SIl19.MiF530cXnUmNnhiFiTPxlybq2pBCP-mTLhJklzBXkPU";
    public static String CUSTOMER_TOKEN = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZW1haWwiOiJqb2huQGRvZS5uZXQiLCJyb2xlIjpbIkNVU1RPTUVSIl19.A3ZjqPqg2CIkP_Dwwc8Uv3WYYmwNr8wXIA-xrUsZ20s";

    public static Role OPERATOR = new Role("OPERATOR", Arrays.asList(Permissions.PARKING_CONTROL));
    public static Role CUSTOMER = new Role("CUSTOMER", Arrays.asList());


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
