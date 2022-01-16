package org.tby.fourdk.core.externalevent;

import org.junit.jupiter.api.Test;
import org.tby.fourdk.core.command.CommandId;
import test.fake.command.ACommand;
import test.fake.externalevent.*;

import java.time.ZonedDateTime;
import java.util.Arrays;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.catchThrowable;
import static test.referential.TestReferential.APR_18_2020_AT_1100AM_CET;

public class ExternalEventDispatcherTest {

    @Test
    public void it_should_dispatch_an_external_event_to_the_correct_policyhandler() {
        // Given
        var anExternalEvent = new AnExternalEvent(APR_18_2020_AT_1100AM_CET);
        var anotherExternalEvent = new AnotherExternalEvent(APR_18_2020_AT_1100AM_CET);
        var aPolicyHandler = new APolicyHandler();
        var anotherPolicyHandler = new AnotherPolicyHandler();
        var externalEventDispatcher = new ExternalEventDispatcher(Arrays.asList(aPolicyHandler, anotherPolicyHandler));

        // When
        externalEventDispatcher.dispatch(anExternalEvent);
        externalEventDispatcher.dispatch(anotherExternalEvent);

        // Then
        assertThat(aPolicyHandler.hasHandled(anExternalEvent)).isTrue();
        assertThat(anotherPolicyHandler.hasHandled(anotherExternalEvent)).isTrue();
    }

    @Test
    public void it_should_throw_an_exception_if_the_event_dispatched_has_no_associated_handler() {
        // Given
        var anExternalEvent = new AnExternalEvent(ZonedDateTime.now());
        var anotherPolicyHandler = new AnotherPolicyHandler();
        var externalEventDispatcher = new ExternalEventDispatcher(Arrays.asList(anotherPolicyHandler));

        // When
        var raisedException = catchThrowable(() -> externalEventDispatcher.dispatch(anExternalEvent));

        // Then
        assertThat(raisedException).isNotNull().isInstanceOf(UnhandledExternalEventException.class);
    }

    @Test
    public void it_should_return_commands_from_the_correct_policy_handler() {
        // Given
        var aCommand = new ACommand();

        var anExternalEvent = new AnExternalEvent(ZonedDateTime.now());
        var aThirdPolicyHandler = new AThirdPolicyHandler(aCommand);
        var externalEventDispatcher = new ExternalEventDispatcher(Arrays.asList(aThirdPolicyHandler));


        // When
        var commands = externalEventDispatcher.dispatch(anExternalEvent);

        // Then
        assertThat(commands).containsAll(Arrays.asList(aCommand));
    }

}
