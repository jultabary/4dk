package test.referential;

import org.tby.fourdk.security.Permission;
import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.userinfo.UserId;

import java.time.ZonedDateTime;
import java.util.Arrays;

public class TestReferential {

    // Datetimes
    public static final ZonedDateTime APR_17_1991_AT_1100AM_CET = ZonedDateTime.parse("1991-04-17T11:00:00+01:00[Europe/Paris]");
    public static final ZonedDateTime APR_18_2020_AT_1100AM_CET = ZonedDateTime.parse("1991-04-18T11:00:00+01:00[Europe/Paris]");

    // User
    public static UserId JUNIT_USER = new UserId("JUNIT_USER");

    // Permissions
    public static final Permission PARKING_CONTROL = new Permission("PARKING_CONTROL");
    public static final Permission PARKING_READ = new Permission("PARKING_READ");

    // Roles
    public static final Role OPERATOR = new Role("OPERATOR", Arrays.asList(PARKING_CONTROL, PARKING_READ));
    public static final Role CUSTOMER = new Role("CUSTOMER", Arrays.asList(PARKING_READ));

}
