package test.fake.event;

public class AnotherEventHandler extends VerifiableEventHandler<AnotherEvent> {
    @Override
    public Class<AnotherEvent> getEventType() {
        return AnotherEvent.class;
    }
}
