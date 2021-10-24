package org.tby.fourdk.security;

import java.util.ArrayList;
import java.util.List;

public class AppPermissions {
    private List<Permission> allPermissions;

    private List<Permission> newPermissions;

    private List<Permission> permissionsToDelete;

    public AppPermissions(List<Permission> allPermissions) {
        this.allPermissions = allPermissions;
    }

    public void definedWhichPermissionsHaveToBeAdded(List<Permission> newPermissionsFound) {
        var permissionsFoundCopy = new ArrayList<>(newPermissionsFound);
        permissionsFoundCopy.removeAll(allPermissions);
        this.newPermissions = permissionsFoundCopy;
    }

    public void definedWhichPermissionsHasToBeRemoved(List<Permission> newPermissionsFound) {
        var allPermissionsCopy = new ArrayList<>(this.allPermissions);
        allPermissionsCopy.removeAll(newPermissionsFound);
        this.permissionsToDelete = allPermissionsCopy;
    }

    public List<Permission> getNewPermissions() {
        if (newPermissions == null) {
            throw new AppPermissionsException("newPermissions have not been initialized, use setter before");
        }
        return newPermissions;
    }

    public List<Permission> getPermissionsToDelete() {
        if (permissionsToDelete == null ) {
            throw new AppPermissionsException("permissionsToDelete have not been initialized, use setter before");
        }
        return permissionsToDelete;
    }

    public Boolean isThereNewPermissions() {
        return newPermissions != null;
    }

    public Boolean isTherePermissionsToDelete() {
        return permissionsToDelete != null;
    }

    public List<Permission> getAllPermissions() {
        return allPermissions;
    }

}
