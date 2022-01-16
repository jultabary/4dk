package org.tby.fourdk.security;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;

public class Authorization {

    public final Boolean isAuthorized;

    public final List<Permission> permissions;

    public Authorization(Boolean isAuthorized, Collection<Permission> permissions) {
        this.isAuthorized = isAuthorized;
        this.permissions = new ArrayList<>(permissions);
    }

}
