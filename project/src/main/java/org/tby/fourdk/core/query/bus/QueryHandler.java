package org.tby.fourdk.core.query.bus;

import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;

public interface QueryHandler<Q extends Query> {

    Response ask(Q query);

    Class<Q> getQueryType();

}
