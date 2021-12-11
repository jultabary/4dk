package org.tby.fourdk.core.query.bus;

import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;

public interface QueryBus {

    Response dispatch(Query query);
}
