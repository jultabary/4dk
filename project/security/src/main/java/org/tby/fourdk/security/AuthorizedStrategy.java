package org.tby.fourdk.security;

import java.util.List;

public interface AuthorizedStrategy {
    Authorization isAuthorized(Permission expectedPermission, List<Role> givenRoles);
}
