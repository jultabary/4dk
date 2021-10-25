package org.tby.fourdk.security.sample.spring.infrastructure.persistence;

import org.springframework.stereotype.Component;
import org.tby.fourdk.security.*;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Optional;

@Component
public class RoleInMemoryRepository implements RoleReadRepository, RoleWritteRepository {

    private List<Role> roles ;

    private List<Permission> permissions;

    public RoleInMemoryRepository() {
        this.roles = new ArrayList<>();
    }

    @Override
    public List<Role> findAllRoles() {
        return roles;
    }

    @Override
    public AppPermissions findAllPermissions() {
        return new AppPermissions(permissions);
    }

    @Override
    public Optional<Role> findRole(Role role) {
        return Optional.empty();
    }

    @Override
    public List<Permission> findPermissionsForGivenRoles(Role role) {
        Optional<Role> optionalRole = this.roles.stream().filter(r -> r.getName().equals(role.getName())).findFirst();
        if (optionalRole.isEmpty()) {
            return Collections.emptyList();
        }
        return optionalRole.get().getAssociatedPermissions();
    }

    @Override
    public void save(Role role) {
        this.roles.add(role);
    }

    @Override
    public void delete(Role role) {
        this.roles.remove(role);
    }

    @Override
    public void save(Permission permission) {
        this.permissions.add(permission);
    }

    @Override
    public void delete(Permission permission) {
        this.permissions.remove(permission);
    }

    public void reset() {
        this.permissions = new ArrayList<>();
        this.roles = new ArrayList<>();
    }
}
