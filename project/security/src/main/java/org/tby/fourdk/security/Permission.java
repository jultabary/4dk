package org.tby.fourdk.security;

import java.util.Objects;

public record Permission(String permission) {

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Permission that = (Permission) o;
        return Objects.equals(permission, that.permission);
    }

    @Override
    public int hashCode() {
        return Objects.hash(permission);
    }

    @Override
    public String toString() {
        return "Permission{" +
                "permission='" + permission + '\'' +
                '}';
    }
}
