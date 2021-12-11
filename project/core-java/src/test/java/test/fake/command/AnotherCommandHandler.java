package test.fake.command;

public class AnotherCommandHandler extends VerifiableCommandHandler<AnotherCommand> {
    @Override
    public Class<AnotherCommand> getCommandType() {
        return AnotherCommand.class;
    }
}
