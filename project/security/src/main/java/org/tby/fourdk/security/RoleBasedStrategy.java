package org.tby.fourdk.security;

import java.util.Collections;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Set;

public class RoleBasedStrategy implements AuthorizedStrategy {
    private RoleReadRepository roleRepository;

    public RoleBasedStrategy(RoleReadRepository roleRepository) {
        this.roleRepository = roleRepository;
    }

    @Override
    public Authorization isAuthorized(Permission expectedPermission, List<Role> givenRoles) {
        if (givenRoles == null || givenRoles.isEmpty()) {
            return new Authorization(false, Collections.emptyList());
        }
        var permissions = findPermissionsForGivenRoles(givenRoles);
        return new Authorization(permissions.contains(expectedPermission), permissions);
    }

    private Set<Permission> findPermissionsForGivenRoles(List<Role> informedRoles) {
        Set<Permission> permissions = new LinkedHashSet<>();
        for (Role informedRole : informedRoles) {
            var associatedPermissions = this.roleRepository.findPermissionsForGivenRoles(informedRole);
            if (associatedPermissions != null) {
                permissions.addAll(associatedPermissions);
            }
        }
        return permissions;
    }
}
