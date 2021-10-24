package test.fake.query;

import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;
import org.tby.fourdk.core.query.bus.QueryHandler;

import java.util.ArrayList;
import java.util.List;

public abstract class VerifiableQueryHandler<Q extends Query> implements QueryHandler<Q> {

    protected List<Query> handledQueries;

    public VerifiableQueryHandler() {
        this.handledQueries = new ArrayList<>();
    }

    @Override
    public Response ask(Q query) {
        this.handledQueries.add(query);
        return new AResponse();
    }

    public boolean hasHandled(Query query) {
        return this.handledQueries.contains(query);
    }
}
