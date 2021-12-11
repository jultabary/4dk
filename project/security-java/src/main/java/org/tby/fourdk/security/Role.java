package org.tby.fourdk.security;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.UUID;

public class Role {

    private String name;

    private List<Permission> associatedPermissions;

    public Role(String role) {
        this.name = role;
    }

    public Role(String name, List<Permission> associatedPermissions) {
        this.associatedPermissions = associatedPermissions;
        this.name = name;
    }

    public boolean isThereSomeUnknownPermissions(List<Permission> permissions) {
        return !permissions.containsAll(this.associatedPermissions);
    }

    public UUID getId() {
        return UUID.nameUUIDFromBytes(this.name.getBytes());
    }


    public String getName() {
        return name;
    }

    public List<Permission> getAssociatedPermissions() {
        return associatedPermissions != null ? associatedPermissions : new ArrayList<>();
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Role role = (Role) o;
        return Objects.equals(name, role.name) && Objects.equals(associatedPermissions, role.associatedPermissions);
    }

    @Override
    public int hashCode() {
        return Objects.hash(name, associatedPermissions);
    }

    @Override
    public String toString() {
        return "Role{" +
                "name='" + name + '\'' +
                ", associatedPermissions=" + associatedPermissions +
                '}';
    }
}
