package org.tby.fourdk.security.query;

import org.junit.jupiter.api.Test;
import org.tby.fourdk.security.ForbiddenAccessException;
import org.tby.fourdk.security.NoSecurityStrategy;
import org.tby.fourdk.security.RoleBasedStrategy;
import test.fake.query.AQuery;
import test.fake.query.AQueryHandler;
import test.fake.query.ASecuredQuery;
import test.fake.query.ASecuredQueryHandler;
import test.fake.role.RoleRepositoryFixture;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.catchThrowable;
import static test.referential.TestReferential.*;

public class SecuredQueryDispatcherTest {

    @Test
    public void it_should_authorized_secured_query_without_roles_to_dispatch_because_security_is_not_enabled() {
        // Given
        RoleRepositoryFixture roleRepositoryFixture = new RoleRepositoryFixture();
        var noSecurityStrategy = new NoSecurityStrategy(roleRepositoryFixture);
        var aSecuredQueryHandler = new ASecuredQueryHandler();
        var aQueryHandler = new AQueryHandler();
        var queryHandlers = new ArrayList();
        queryHandlers.add(aSecuredQueryHandler);
        queryHandlers.add(aQueryHandler);
        var securedQueryDispatcher = new SecuredQueryDispatcher(noSecurityStrategy, queryHandlers);
        var aSecuredQuery = new ASecuredQuery(Collections.emptyList(), JUNIT_USER);

        // When
        var exception = catchThrowable(() -> securedQueryDispatcher.dispatch(aSecuredQuery));

        // Then
        assertThat(exception).isNull();
        assertThat(aSecuredQueryHandler.hasHandled(aSecuredQuery)).isTrue();
    }

    @Test
    public void it_should_raised_an_forbidden_exception_when_secured_query_has_no_permissions() {
        // Given
        var roleBasedStrategy = new RoleBasedStrategy(new RoleRepositoryFixture());
        var aSecuredQueryHandler = new ASecuredQueryHandler();
        var aQueryHandler = new AQueryHandler();
        var queryHandlers = new ArrayList();
        queryHandlers.add(aSecuredQueryHandler);
        queryHandlers.add(aQueryHandler);
        var securedQueryDispatcher = new SecuredQueryDispatcher(roleBasedStrategy, queryHandlers);
        var aSecuredQuery = new ASecuredQuery(Collections.emptyList(), JUNIT_USER);

        // When
        var exception = catchThrowable(() -> securedQueryDispatcher.dispatch(aSecuredQuery));

        // Then
        assertThat(exception).isNotNull();
        assertThat(exception).isInstanceOf(ForbiddenAccessException.class);
    }

    @Test
    public void it_should_raised_an_forbidden_exception_when_query_has_not_correct_permissions() {
        // Given
        var roleBasedStrategy = new RoleBasedStrategy(new RoleRepositoryFixture());
        var aSecuredQueryHandler = new ASecuredQueryHandler();
        var aQueryHandler = new AQueryHandler();
        var queryHandlers = new ArrayList();
        queryHandlers.add(aSecuredQueryHandler);
        queryHandlers.add(aQueryHandler);
        var securedQueryDispatcher = new SecuredQueryDispatcher(roleBasedStrategy, queryHandlers);
        var aSecuredQuery = new ASecuredQuery(Arrays.asList(CUSTOMER), JUNIT_USER);

        // When
        var exception = catchThrowable(() -> securedQueryDispatcher.dispatch(aSecuredQuery));

        // Then
        assertThat(exception).isNotNull();
        assertThat(exception).isInstanceOf(ForbiddenAccessException.class);
        assertThat(aSecuredQueryHandler.hasHandled(aSecuredQuery)).isFalse();
    }


    @Test
    public void it_should_authorize_dispatch_when_query_has_correct_permissions() {
        // Given
        var roleBasedStrategy = new RoleBasedStrategy(new RoleRepositoryFixture());
        var aSecuredQueryHandler = new ASecuredQueryHandler();
        var aQueryHandler = new AQueryHandler();
        var queryHandlers = new ArrayList();
        queryHandlers.add(aSecuredQueryHandler);
        queryHandlers.add(aQueryHandler);
        var securedQueryDispatcher = new SecuredQueryDispatcher(roleBasedStrategy, queryHandlers);
        var aSecuredQuery = new ASecuredQuery(Arrays.asList(OPERATOR), JUNIT_USER);

        // When
        var exception = catchThrowable(() -> securedQueryDispatcher.dispatch(aSecuredQuery));

        // Then
        assertThat(exception).isNull();
        assertThat(aSecuredQueryHandler.hasHandled(aSecuredQuery)).isTrue();
        assertThat(aSecuredQuery.getPermissions()).contains(PARKING_CONTROL, PARKING_READ);
    }

    @Test
    public void it_should_authorize_dispatch_when_query_is_not_a_secured_query() {
        // Given
        var roleBasedStrategy = new RoleBasedStrategy(new RoleRepositoryFixture());
        var aSecuredQueryHandler = new ASecuredQueryHandler();
        var aQueryHandler = new AQueryHandler();
        var queryHandlers = new ArrayList();
        queryHandlers.add(aSecuredQueryHandler);
        queryHandlers.add(aQueryHandler);
        var securedQueryDispatcher = new SecuredQueryDispatcher(roleBasedStrategy, queryHandlers);
        var aQuery = new AQuery();

        // When
        var exception = catchThrowable(() -> securedQueryDispatcher.dispatch(aQuery));

        // Then
        assertThat(exception).isNull();
        assertThat(aQueryHandler.hasHandled(aQuery)).isTrue();
    }

}
