package org.tby.fourdk.security;

import java.util.List;
import java.util.Optional;

public interface RoleReadRepository {

    List<Role> findAllRoles();

    AppPermissions findAllPermissions();

    Optional<Role> findRole(Role role);

    List<Permission> findPermissionsForGivenRoles(Role role);

}
