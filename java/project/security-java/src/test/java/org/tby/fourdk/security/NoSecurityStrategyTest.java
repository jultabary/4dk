package org.tby.fourdk.security;

import org.junit.jupiter.api.Test;
import test.fake.role.RoleRepositoryFixture;

import java.util.Collections;

import static org.assertj.core.api.Assertions.assertThat;
import static test.referential.TestReferential.PARKING_CONTROL;
import static test.referential.TestReferential.PARKING_READ;

public class NoSecurityStrategyTest {

    @Test
    public void it_should_always_authorized_the_call() {
        // Given
        var roleRepository = new RoleRepositoryFixture();
        var strategy = new NoSecurityStrategy(roleRepository);

        // When
        var authorization = strategy.isAuthorized(PARKING_CONTROL, Collections.emptyList());

        // Then
        assertThat(authorization.isAuthorized).isTrue();
        assertThat(authorization.permissions).contains(PARKING_CONTROL, PARKING_READ);
    }
}
