package test.fake.query;

public class AQueryHandler extends VerifiableQueryHandler<AQuery> {

    @Override
    public Class<AQuery> getQueryType() {
        return AQuery.class;
    }
}
