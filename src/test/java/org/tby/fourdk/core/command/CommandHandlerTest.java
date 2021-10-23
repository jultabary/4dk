package org.tby.fourdk.core.command;

import org.junit.jupiter.api.Test;
import test.fake.command.ACommandHandler;

import static org.assertj.core.api.Assertions.assertThat;

class CommandHandlerTest {

    @Test
    public void name_should_be_equals_to_class_simple_name() {
        // Given
        var commandHandler = new ACommandHandler();

        // When
        var name = commandHandler.getName();

        // Then
        assertThat(name).isEqualTo("ACommandHandler");
    }


}