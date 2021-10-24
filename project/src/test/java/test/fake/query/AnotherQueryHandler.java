package test.fake.query;

public class AnotherQueryHandler extends VerifiableQueryHandler<AnotherQuery> {
    @Override
    public Class<AnotherQuery> getQueryType() {
        return AnotherQuery.class;
    }
}
