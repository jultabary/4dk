package test.fake.query;

import org.tby.fourdk.core.query.Response;

public class AThirdQueryHandler extends VerifiableQueryHandler<AQuery> {

    private AResponse aResponse;

    public AThirdQueryHandler(AResponse aQueryResponse) {
        this.aResponse = aQueryResponse;
    }

    @Override
    public Response ask(AQuery query) {
        super.ask(query);
        return this.aResponse;
    }

    @Override
    public Class<AQuery> getQueryType() {
        return AQuery.class;
    }
}
