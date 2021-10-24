package test.fake.command;

import org.tby.fourdk.security.Permission;

import static test.referential.TestReferential.PARKING_CONTROL;

public class ASecuredCommandHandler extends VerifiableSecuredCommandHandler<ASecuredCommand> {

    @Override
    public Class<ASecuredCommand> getCommandType() {
        return ASecuredCommand.class;
    }

    @Override
    public Permission getAssociatedPermission() {
        return PARKING_CONTROL;
    }
}
