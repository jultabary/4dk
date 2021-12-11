package test.fake.query;

import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.query.SecuredQuery;
import org.tby.fourdk.security.userinfo.UserId;

import java.util.List;

public class ASecuredQuery extends SecuredQuery {
    public ASecuredQuery(List<Role> userRoles, UserId userId) {
        super(userRoles, userId);
    }
}
