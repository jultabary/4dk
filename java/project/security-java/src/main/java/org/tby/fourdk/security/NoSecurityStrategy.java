package org.tby.fourdk.security;

import java.util.List;

public class NoSecurityStrategy implements AuthorizedStrategy {

    private RoleReadRepository roleRepository;

    public NoSecurityStrategy(RoleReadRepository roleRepository) {
        this.roleRepository = roleRepository;
    }

    @Override
    public Authorization isAuthorized(Permission expectedPermission, List<Role> roles) {
        var appPermissions = this.roleRepository.findAllPermissions();
        return new Authorization(true, appPermissions.getAllPermissions());
    }


}
