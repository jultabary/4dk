package test.fake.query;

import org.tby.fourdk.security.Permission;

import static test.referential.TestReferential.PARKING_CONTROL;

public class ASecuredQueryHandler extends VerifiableSecuredQueryHandler<ASecuredQuery> {
    @Override
    public Class<ASecuredQuery> getQueryType() {
        return ASecuredQuery.class;
    }

    @Override
    public Permission getAssociatedPermission() {
        return PARKING_CONTROL;
    }
}
