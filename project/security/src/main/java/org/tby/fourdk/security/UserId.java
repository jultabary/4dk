package org.tby.fourdk.security;

import java.util.Objects;

public class UserId {

    private String login;

    public UserId(String login) {
        this.login = login;
    }

    public String getLogin() {
        return login;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        UserId userId = (UserId) o;
        return Objects.equals(login, userId.login);
    }

    @Override
    public int hashCode() {
        return Objects.hash(login);
    }

    @Override
    public String toString() {
        return "UserId{" +
                "login='" + login + '\'' +
                '}';
    }

}
