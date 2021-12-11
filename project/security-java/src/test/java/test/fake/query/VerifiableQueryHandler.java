package test.fake.query;

import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;
import org.tby.fourdk.core.query.bus.QueryHandler;

import java.util.ArrayList;
import java.util.List;

public abstract class VerifiableQueryHandler<Q extends Query> implements QueryHandler<Q> {

    private List<Query> handledQueries;

    public VerifiableQueryHandler() {
        this.handledQueries = new ArrayList<>();
    }

    @Override
    public Response ask(Q query) {
        handledQueries.add(query);
        return new AResponse();
    }

    public Boolean hasHandled(Query q) {
        return this.handledQueries.contains(q);
    }

}
