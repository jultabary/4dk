package test.fake.event;

public class AnEventHandler extends VerifiableEventHandler<AnEvent> {

    @Override
    public Class<AnEvent> getEventType() {
        return AnEvent.class;
    }
}
