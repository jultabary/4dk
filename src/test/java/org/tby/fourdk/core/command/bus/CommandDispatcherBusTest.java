package org.tby.fourdk.core.command.bus;

import org.junit.jupiter.api.Test;
import org.tby.fourdk.core.command.CommandId;
import org.tby.fourdk.core.event.EventId;
import test.fake.command.*;
import test.fake.event.AnEvent;

import java.util.Arrays;
import java.util.Collections;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.catchThrowable;
import static test.referential.TestReferential.APR_17_1991_AT_1100AM_CET;

class CommandDispatcherBusTest {

    @Test
    public void it_should_dispatch_a_command_to_the_correct_commandHandler() {
        // Given
        var aCommand = new ACommand();
        var anotherCommand = new AnotherCommand();
        var aCommandHandler = new ACommandHandler();
        var anotherCommandHandler = new AnotherCommandHandler();
        var commandDispatcher = new CommandDispatcher(Arrays.asList(aCommandHandler, anotherCommandHandler));

        // When
        commandDispatcher.dispatch(aCommand);
        commandDispatcher.dispatch(anotherCommand);

        // Then
        assertThat(aCommandHandler.hasHandled(aCommand)).isTrue();
        assertThat(anotherCommandHandler.hasHandled(anotherCommand)).isTrue();
    }

    @Test
    public void it_should_throw_an_exception_if_the_command_dispatched_has_no_associated_handler() {
        // Given
        var aCommand = new ACommand();
        var anotherCommandHandler = new AnotherCommandHandler();
        var CommandDispatcherBus = new CommandDispatcher(Arrays.asList(anotherCommandHandler));

        // When
        var raisedException = catchThrowable(() -> CommandDispatcherBus.dispatch(aCommand));

        // Then
        assertThat(raisedException).isNotNull().isInstanceOf(UnhandledCommandException.class);
    }

    @Test
    public void it_should_only_register_one_handler_for_a_given_command() {
        // Given
        var aCommandHandlerWhichHandledACommand = new ACommandHandler();
        var anotherCommandHandlerWhichHandledACommand = new CommandHandlerReturningAnEvent(Collections.emptyList());

        // When
        var raisedException = catchThrowable(() -> new CommandDispatcher(Arrays.asList(aCommandHandlerWhichHandledACommand, anotherCommandHandlerWhichHandledACommand)));

        // Then
        assertThat(raisedException).isNotNull().isInstanceOf(CommandAlreadyHandledException.class);
    }

    @Test
    public void it_should_return_events_from_the_correct_command_handler() {
        // Given
        var commandId = CommandId.create();

        var event = new AnEvent(commandId, APR_17_1991_AT_1100AM_CET);
        var aCommand = new ACommand();
        var anotherCommandHandler = new AnotherCommandHandler();
        var aThirdCommandHandler = new CommandHandlerReturningAnEvent(Arrays.asList(event));
        var CommandDispatcherBus = new CommandDispatcher(Arrays.asList(anotherCommandHandler, aThirdCommandHandler));

        // When
        var events = CommandDispatcherBus.dispatch(aCommand);

        // Then
        assertThat(events).containsAll(Arrays.asList(event));
    }

}