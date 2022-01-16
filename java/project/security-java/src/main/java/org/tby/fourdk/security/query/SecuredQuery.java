package org.tby.fourdk.security.query;

import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.security.Permission;
import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.userinfo.UserId;

import java.util.List;

public class SecuredQuery implements Query {

    public final List<Role> userRoles;

    public final UserId userId;

    private List<Permission> userPermissions;

    public SecuredQuery(List<Role> userRoles, UserId userId) {
        this.userRoles = userRoles;
        this.userId = userId;
    }

    public List<Permission> getPermissions() {
        return userPermissions;
    }

    public void setPermissions(List<Permission> permissions) {
        this.userPermissions = permissions;
    }
}
