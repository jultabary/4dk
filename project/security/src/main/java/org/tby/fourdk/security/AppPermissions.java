package org.tby.fourdk.security;

import java.util.ArrayList;
import java.util.List;

public class AppPermissions {
    private List<Permission> allPermissions;

    public AppPermissions(List<Permission> allPermissions) {
        this.allPermissions = allPermissions;
    }

    public List<Permission> getAllPermissions() {
        return allPermissions;
    }

}
