package test.fake.role;

import org.tby.fourdk.security.AppPermissions;
import org.tby.fourdk.security.Permission;
import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.RoleRepository;

import java.util.*;

import static test.referential.TestReferential.CUSTOMER;
import static test.referential.TestReferential.OPERATOR;

public class RoleRepositoryFixture implements RoleRepository {

    private Integer findAllRolesCount;

    private List<Role> roles;

    public RoleRepositoryFixture() {
        this.roles = new ArrayList<>();
        this.roles.add(OPERATOR);
        this.roles.add(CUSTOMER);
        findAllRolesCount = 0;
    }

    @Override
    public List<Role> findAllRoles() {
        findAllRolesCount++;
        return roles;
    }

    @Override
    public AppPermissions findAllPermissions() {
        Set<Permission> allPermissions = new HashSet<>();
        allPermissions.addAll(OPERATOR.getAssociatedPermissions());
        allPermissions.addAll(CUSTOMER.getAssociatedPermissions());
        return new AppPermissions(allPermissions.stream().toList());
    }

    @Override
    public Optional<Role> findRole(Role role) {
        return this.findAllRoles().stream().filter(r -> r.getId().equals(role.getId())).findFirst();
    }

    @Override
    public List<Permission> findPermissionsForGivenRoles(Role role) {
        Optional<Role> roleOpt = this.findRole(role);
        if (roleOpt.isPresent()) {
            return roleOpt.get().getAssociatedPermissions();
        }
        return new ArrayList<>();
    }

    public Integer getFindAllRolesCount() {
        return findAllRolesCount;
    }

}
