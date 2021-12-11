package test.fake.externalevent;

public class APolicyHandler extends PolicyHandlerVerifiable<AnExternalEvent> {

    @Override
    public Class getEventType() {
        return AnExternalEvent.class;
    }
}
