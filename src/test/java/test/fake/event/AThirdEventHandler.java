package test.fake.event;

public class AThirdEventHandler extends VerifiableEventHandler<AnEvent>{
    @Override
    public Class<AnEvent> getEventType() {
        return AnEvent.class;
    }
}
