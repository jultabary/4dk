package org.tby.fourdk.security;

import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.UUID;

import static org.assertj.core.api.Assertions.assertThat;
import static test.referential.TestReferential.PARKING_CONTROL;
import static test.referential.TestReferential.PARKING_READ;

public class RoleTest {

    @Test
    public void constructor_should_well_instantiate_role_object() {
        // Given
        var roleName = "ROLE";

        // When
        Role role = new Role(roleName);

        // Then
        assertThat(role.getName()).isEqualTo(roleName);
        assertThat(role.getId()).isEqualTo(UUID.nameUUIDFromBytes(roleName.getBytes()));
    }

    @Test
    public void is_should_process_that_it_contains_unknown_permissions() {
        // Given
        var roleName = "ROLE";
        var existingPermissions = Arrays.asList(PARKING_READ);
        var givenPermissions = Arrays.asList(PARKING_CONTROL);
        Role role = new Role(roleName, givenPermissions);

        // When
        var result = role.isThereSomeUnknownPermissions(existingPermissions);
        // Then
        assertThat(result).isTrue();
    }

    @Test
    public void is_should_process_that_it_contains_no_unknown_permissions() {
        // Given
        var roleName = "ROLE";
        var existingPermissions = Arrays.asList(PARKING_CONTROL);
        Role role = new Role(roleName,  existingPermissions);

        // When
        var result = role.isThereSomeUnknownPermissions(existingPermissions);
        // Then
        assertThat(result).isFalse();
    }

}
