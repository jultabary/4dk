package org.tby.fourdk.security.command;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.security.Permission;
import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.userinfo.UserId;

import java.util.List;

public abstract class SecuredCommand extends Command {

    public final List<Role> userRoles;

    public final UserId userId;

    private List<Permission> userPermissions;

    public SecuredCommand(List<Role> roles, UserId userId) {
        super();
        this.userRoles = roles;
        this.userId = userId;
    }

    public List<Permission> getPermissions() {
        return userPermissions;
    }

    public void setPermissions(List<Permission> permissions) {
        this.userPermissions = permissions;
    }

}
