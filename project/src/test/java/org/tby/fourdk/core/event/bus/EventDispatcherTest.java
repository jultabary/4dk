package org.tby.fourdk.core.event.bus;

import org.junit.jupiter.api.Test;
import test.fake.event.*;

import java.util.Arrays;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.catchThrowable;
import static test.referential.TestReferential.APR_17_1991_AT_1100AM_CET;

class EventDispatcherTest {
    @Test
    public void it_could_register_several_handlers_for_a_given_event() {
        // Given
        var anEventHandler = new AnEventHandler();
        var anotherEventHandlerHandlingTheSameEvent = new AThirdEventHandler();

        // When
        var raisedException = catchThrowable(() -> new EventDispatcher(Arrays.asList(anEventHandler, anotherEventHandlerHandlingTheSameEvent)));

        // Then
        assertThat(raisedException).isNull();

    }

    @Test
    public void it_should_not_register_an_eventhandler_twice() {
        // Given
        var anEventHandler = new AnEventHandler();
        var anotherEventHandler = new AnEventHandler();

        // When
        var raisedException = catchThrowable(() -> new EventDispatcher(Arrays.asList(anEventHandler, anotherEventHandler)));

        // Then
        assertThat(raisedException).isNotNull();
        assertThat(raisedException).isInstanceOf(EventAlreadyHandledException.class);

    }


    @Test
    public void it_should_dispatch_a_event_to_the_correct_eventhandlers() {
        // Given
        var anEvent = new AnEvent(APR_17_1991_AT_1100AM_CET);
        var anEventHandler = new AnEventHandler();
        var anotherEventHandlerHandlingTheSameEvent = new AThirdEventHandler();
        var anotherEventHandler = new AnotherEventHandler();
        var eventDispatcher = new EventDispatcher(Arrays.asList(anEventHandler, anotherEventHandlerHandlingTheSameEvent, anotherEventHandler));

        // When
        eventDispatcher.dispatch(anEvent);

        // Then
        assertThat(anEventHandler.hasHandled(anEvent)).isTrue();
        assertThat(anotherEventHandlerHandlingTheSameEvent.hasHandled(anEvent)).isTrue();
        assertThat(anotherEventHandler.hasHandled(anEvent)).isFalse();
    }

    @Test
    public void it_should_ignore_unhandled_event() {
        // Given
        var anEvent = new AFourthEvent(APR_17_1991_AT_1100AM_CET);
        var anEventHandler = new AnEventHandler();
        var anotherEventHandlerHandlingTheSameEvent = new AThirdEventHandler();
        var anotherEventHandler = new AnotherEventHandler();
        var eventDispatcher = new EventDispatcher(Arrays.asList(anEventHandler, anotherEventHandlerHandlingTheSameEvent, anotherEventHandler));

        // When
        eventDispatcher.dispatch(anEvent);

        // Then
        assertThat(anEventHandler.hasHandled(anEvent)).isFalse();
        assertThat(anotherEventHandlerHandlingTheSameEvent.hasHandled(anEvent)).isFalse();
        assertThat(anotherEventHandler.hasHandled(anEvent)).isFalse();
    }

}