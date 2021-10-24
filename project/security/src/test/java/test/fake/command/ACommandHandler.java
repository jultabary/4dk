package test.fake.command;

public class ACommandHandler extends VerifiableCommandHandler<ACommand> {

    @Override
    public Class<ACommand> getCommandType() {
        return ACommand.class;
    }
}
