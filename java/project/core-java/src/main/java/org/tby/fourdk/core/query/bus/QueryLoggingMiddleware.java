package org.tby.fourdk.core.query.bus;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;

public class QueryLoggingMiddleware implements QueryBus{
    private static final Logger LOGGER = LoggerFactory.getLogger(QueryLoggingMiddleware.class);

    private QueryBus queryBus;

    public QueryLoggingMiddleware(QueryBus queryBus) {
        this.queryBus = queryBus;
    }

    @Override
    public Response dispatch(Query query) {
        var begin =  System.currentTimeMillis();

        var type = query.getClass().getSimpleName();
        LOGGER.info("Handling query of type [{}] [{}]", type, query);
        Response queryResponse;
        try {
            queryResponse = this.queryBus.dispatch(query);
        } catch (RuntimeException e) {
            LOGGER.info("An error occurred while handling query [{}] [{}]", type, query);
            throw e;
        }
        var end = System.currentTimeMillis();

        LOGGER.info("Query of type [{}] [{}] has been handled in [{}] ms.",
                type, query.toString(), queryHandlerProcessTime(begin, end));

        return queryResponse;
    }

    private long queryHandlerProcessTime(long begin, long end) {
        return end - begin;
    }
}
