package org.tby.fourdk.security;

import org.junit.jupiter.api.Test;
import test.fake.command.ASecuredCommand;
import test.fake.command.ASecuredCommandHandler;
import test.fake.role.RoleRepositoryFixture;

import java.util.Arrays;

import static org.assertj.core.api.Assertions.assertThat;
import static test.referential.TestReferential.*;

public class RoleBasedStrategyTest {

    @Test
    public void it_should_authorized_command_for_given_roles() {
        // Given
        RoleRepositoryFixture roleRepositoryFixture = new RoleRepositoryFixture();
        RoleBasedStrategy roleBasedStrategy = new RoleBasedStrategy(roleRepositoryFixture);
        ASecuredCommand aSecuredCommand = new ASecuredCommand(Arrays.asList(OPERATOR, CUSTOMER), JUNIT_USER);
        ASecuredCommandHandler aSecuredCommandHandler = new ASecuredCommandHandler();

        // When
        var authorization = roleBasedStrategy.isAuthorized(aSecuredCommandHandler.getAssociatedPermission(), aSecuredCommand.userRoles);

        // Then
        assertThat(authorization.isAuthorized).isTrue();
        assertThat(authorization.permissions).contains(PARKING_CONTROL, PARKING_READ);
    }

    @Test
    public void it_should_not_authorized_command_for_given_roles() {
        // Given
        RoleRepositoryFixture roleRepositoryFixture = new RoleRepositoryFixture();
        RoleBasedStrategy roleBasedStrategy = new RoleBasedStrategy(roleRepositoryFixture);
        ASecuredCommand aSecuredCommand = new ASecuredCommand(Arrays.asList(CUSTOMER), JUNIT_USER);
        ASecuredCommandHandler aSecuredCommandHandler = new ASecuredCommandHandler();

        // When
        var authorization = roleBasedStrategy.isAuthorized(aSecuredCommandHandler.getAssociatedPermission(), aSecuredCommand.userRoles);

        // Then
        assertThat(authorization.isAuthorized).isFalse();
    }
}
