package org.tby.fourdk.security.command;

import org.junit.jupiter.api.Test;
import org.tby.fourdk.security.ForbiddenAccessException;
import org.tby.fourdk.security.NoSecurityStrategy;
import org.tby.fourdk.security.RoleBasedStrategy;
import test.fake.command.ACommand;
import test.fake.command.ACommandHandler;
import test.fake.command.ASecuredCommand;
import test.fake.command.ASecuredCommandHandler;
import test.fake.role.RoleRepositoryFixture;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.catchThrowable;
import static test.referential.TestReferential.*;

class SecuredCommandDispatcherTest {

    @Test
    public void it_should_authorized_secured_command_without_roles_to_dispatch_because_security_is_not_enabled() {
        // Given
        RoleRepositoryFixture roleRepositoryFixture = new RoleRepositoryFixture();
        var noSecurityStrategy = new NoSecurityStrategy(roleRepositoryFixture);
        var securedCommandHandler = new ASecuredCommandHandler();
        var aCommandHandler = new ACommandHandler();
        var commandHandlers = new ArrayList();
        commandHandlers.add(securedCommandHandler);
        commandHandlers.add(aCommandHandler);
        var authorizeCommandDispatcher = new SecuredCommandDispatcher(noSecurityStrategy, commandHandlers);
        var aSecuredCommand = new ASecuredCommand(null, JUNIT_USER);

        // When
        var exception = catchThrowable(() -> authorizeCommandDispatcher.dispatch(aSecuredCommand));

        // Then
        assertThat(exception).isNull();
        assertThat(securedCommandHandler.hasHandled(aSecuredCommand)).isTrue();
    }

    @Test
    public void it_should_raised_an_forbidden_exception_when_command_has_no_permissions() {
        // Given
        var roleBasedStrategy = new RoleBasedStrategy(new RoleRepositoryFixture());
        var aSecuredCommandHandler = new ASecuredCommandHandler();
        var aCommandHandler = new ACommandHandler();
        var commandHandlers = new ArrayList();
        commandHandlers.add(aSecuredCommandHandler);
        commandHandlers.add(aCommandHandler);
        var authorizeCommandDispatcher = new SecuredCommandDispatcher(roleBasedStrategy, commandHandlers);
        var aSecuredCommand = new ASecuredCommand(Collections.emptyList(), JUNIT_USER);

        // When
        var exception = catchThrowable(() -> authorizeCommandDispatcher.dispatch(aSecuredCommand));

        // Then
        assertThat(exception).isNotNull();
        assertThat(exception).isInstanceOf(ForbiddenAccessException.class);
    }

    @Test
    public void it_should_raised_an_forbidden_exception_when_command_has_not_correct_permissions() {
        // Given
        var roleBasedStrategy = new RoleBasedStrategy(new RoleRepositoryFixture());
        var aSecuredCommandHandler = new ASecuredCommandHandler();
        var aCommandHandler = new ACommandHandler();
        var commandHandlers = new ArrayList();
        commandHandlers.add(aSecuredCommandHandler);
        commandHandlers.add(aCommandHandler);
        var authorizeCommandDispatcher = new SecuredCommandDispatcher(roleBasedStrategy, commandHandlers);
        var aSecuredCommand = new ASecuredCommand(Arrays.asList(CUSTOMER), JUNIT_USER);

        // When
        var exception = catchThrowable(() -> authorizeCommandDispatcher.dispatch(aSecuredCommand));

        // Then
        assertThat(exception).isNotNull();
        assertThat(exception).isInstanceOf(ForbiddenAccessException.class);
        assertThat(aSecuredCommandHandler.hasHandled(aSecuredCommand)).isFalse();
    }


    @Test
    public void it_should_authorize_dispatch_when_command_has_correct_permissions() {
        // Given
        var roleBasedStrategy = new RoleBasedStrategy(new RoleRepositoryFixture());
        var aSecuredCommandHandler = new ASecuredCommandHandler();
        var aCommandHandler = new ACommandHandler();
        var commandHandlers = new ArrayList();
        commandHandlers.add(aSecuredCommandHandler);
        commandHandlers.add(aCommandHandler);
        var authorizeCommandDispatcher = new SecuredCommandDispatcher(roleBasedStrategy, commandHandlers);
        var aSecuredCommand = new ASecuredCommand(Arrays.asList(OPERATOR), JUNIT_USER);

        // When
        var exception = catchThrowable(() -> authorizeCommandDispatcher.dispatch(aSecuredCommand));

        // Then
        assertThat(exception).isNull();
        assertThat(aSecuredCommandHandler.hasHandled(aSecuredCommand)).isTrue();
        assertThat(aSecuredCommand.getPermissions()).contains(PARKING_CONTROL, PARKING_READ);
    }

    @Test
    public void it_should_authorize_dispatch_when_command_is_not_a_secured_command() {
        // Given
        var roleBasedStrategy = new RoleBasedStrategy(new RoleRepositoryFixture());
        var aSecuredCommandHandler = new ASecuredCommandHandler();
        var aCommandHandler = new ACommandHandler();
        var commandHandlers = new ArrayList();
        commandHandlers.add(aSecuredCommandHandler);
        commandHandlers.add(aCommandHandler);
        var authorizeCommandDispatcher = new SecuredCommandDispatcher(roleBasedStrategy, commandHandlers);
        var aCommand = new ACommand();

        // When
        var exception = catchThrowable(() -> authorizeCommandDispatcher.dispatch(aCommand));

        // Then
        assertThat(exception).isNull();
        assertThat(aCommandHandler.hasHandled(aCommand)).isTrue();
    }

}