package org.tby.fourdk.security.userinfo;

import org.tby.fourdk.security.Role;

import java.util.List;
import java.util.Objects;

public record UserInfo(UserId id, List<Role> roles) {

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        UserInfo userInfo = (UserInfo) o;
        return Objects.equals(id, userInfo.id) && Objects.equals(roles, userInfo.roles);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, roles);
    }

    @Override
    public String toString() {
        return "UserInfo{" +
                "id=" + id +
                ", roles=" + roles +
                '}';
    }
}
