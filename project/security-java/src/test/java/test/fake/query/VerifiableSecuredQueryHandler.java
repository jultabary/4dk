package test.fake.query;

import org.tby.fourdk.core.query.Query;
import org.tby.fourdk.core.query.Response;
import org.tby.fourdk.security.query.SecuredQuery;
import org.tby.fourdk.security.query.SecuredQueryHandler;

import java.util.ArrayList;
import java.util.List;

public abstract class VerifiableSecuredQueryHandler<Q extends SecuredQuery> implements SecuredQueryHandler<Q> {

    private List<Query> handledQueries;

    public VerifiableSecuredQueryHandler() {
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
