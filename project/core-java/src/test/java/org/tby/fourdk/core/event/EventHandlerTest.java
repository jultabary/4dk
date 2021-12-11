package org.tby.fourdk.core.event;

import org.junit.jupiter.api.Test;
import test.fake.event.AnEventHandler;

import static org.assertj.core.api.Assertions.assertThat;

public class EventHandlerTest {

    @Test
    public void name_should_be_equals_to_class_simple_name() {
        // Given
        var eventHandler = new AnEventHandler();

        // When
        var name = eventHandler.getName();

        // Then
        assertThat(name).isEqualTo("AnEventHandler");
    }
}
