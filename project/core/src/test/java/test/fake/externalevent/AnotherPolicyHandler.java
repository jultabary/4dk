package test.fake.externalevent;

public class AnotherPolicyHandler extends PolicyHandlerVerifiable<AnotherExternalEvent> {

    @Override
    public Class getEventType() {
        return AnotherExternalEvent.class;
    }
}
