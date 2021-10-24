package org.tby.fourdk.security;

import java.util.List;

public class NoSecurityStrategy implements AuthorizedStrategy {

    private RoleRepository roleRepository;

    public NoSecurityStrategy(RoleRepository roleRepository) {
        this.roleRepository = roleRepository;
    }

    @Override
    public Authorization isAuthorized(Permission expectedPermission, List<Role> roles) {
        var appPermissions = this.roleRepository.findAllPermissions();
        return new Authorization(true, appPermissions.getAllPermissions());
    }


}
