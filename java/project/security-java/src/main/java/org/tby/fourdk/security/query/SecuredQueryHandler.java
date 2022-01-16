package org.tby.fourdk.security.query;

import org.tby.fourdk.core.query.bus.QueryHandler;
import org.tby.fourdk.security.Permission;

public interface SecuredQueryHandler<Q extends SecuredQuery> extends QueryHandler<Q> {
    Permission getAssociatedPermission();
}
