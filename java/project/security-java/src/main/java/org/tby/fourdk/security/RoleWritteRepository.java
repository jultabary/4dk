package org.tby.fourdk.security;

public interface RoleWritteRepository {

    void save(Role role);

    void delete(Role role);

    void save(Permission permission);

    void delete(Permission permission);
    
}
