package org.tby.fourdk.core.query.bus;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class QueryDispatcher implements QueryBus{
    private static final Logger LOGGER = LoggerFactory.getLogger(QueryDispatcher.class);

    protected Map<Class, QueryHandler> queryHandlers;

    public QueryDispatcher(List<QueryHandler> queryHandlers) {
        this.queryHandlers = new HashMap<>();
        registerQueryHandlers(queryHandlers);
    }


    @Override
    public Response dispatch(Query query) {
        if (!isQueryHandled(query.getClass())) {
            var type = query.getClass().getSimpleName();
            throw new UnhandledQueryException("Query type [" + type + "] is not handled.");
        }
        return this.queryHandlers.get(query.getClass()).ask(query);
    }

    private boolean isQueryHandled(Class queryClass) {
        return this.queryHandlers.containsKey(queryClass);
    }

    private void registerQueryHandlers(List<QueryHandler> queryHandlers) {
        for (QueryHandler queryHandler : queryHandlers) {
            var type = queryHandler.getQueryType().getSimpleName();
            if (isQueryHandled(queryHandler.getQueryType())) {
                throw new QueryAlreadyHandledException("Query type [" + type + "] is already handled." );
            } else {
                LOGGER.info("Register queryHandler [{}] for query [{}]", queryHandler.getClass().getSimpleName(), type);
                this.queryHandlers.put(queryHandler.getQueryType(), queryHandler);
            }
        }
    }
}
