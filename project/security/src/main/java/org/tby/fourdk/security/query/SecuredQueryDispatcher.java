package org.tby.fourdk.security.query;

import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;
import org.tby.fourdk.core.query.bus.QueryDispatcher;
import org.tby.fourdk.core.query.bus.QueryHandler;
import org.tby.fourdk.security.AuthorizedStrategy;
import org.tby.fourdk.security.ForbiddenAccessException;
import org.tby.fourdk.security.Role;

import java.util.ArrayList;
import java.util.List;

public class SecuredQueryDispatcher extends QueryDispatcher {

    private AuthorizedStrategy authorizedStrategy;

    public SecuredQueryDispatcher(AuthorizedStrategy authorizedStrategy, List<QueryHandler> queryHandlers) {
        super(queryHandlers);
        this.authorizedStrategy = authorizedStrategy;
    }

    @Override
    public Response dispatch(Query query) {
        var queryHandler = this.queryHandlers.get(query.getClass());
        List<Role> roles = new ArrayList<Role>();
        if (query instanceof SecuredQuery) {
            roles = ((SecuredQuery) query).userRoles != null ? ((SecuredQuery) query).userRoles : roles;
        }
        if (queryHandler instanceof SecuredQueryHandler) {
            var authorization = authorizedStrategy.isAuthorized(((SecuredQueryHandler) queryHandler).getAssociatedPermission(), roles);
            if (!authorization.isAuthorized) {
                throw new ForbiddenAccessException("Access to [" + queryHandler.getClass().getSimpleName() + "] is forbidden for the given roles");
            }
            if (query instanceof SecuredQuery) {
                ((SecuredQuery) query).setPermissions(new ArrayList<>(authorization.permissions));
            }
        }
        return super.dispatch(query);
    }
}