package org.tby.fourdk.core.command.bus;

import org.junit.jupiter.api.Test;
import org.tby.fourdk.core.command.CommandId;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.event.bus.EventDispatcher;
import test.fake.command.ACommand;
import test.fake.command.CommandHandlerReturningAnEvent;
import test.fake.event.AThirdEventHandler;
import test.fake.event.AnEvent;
import test.fake.event.AnEventHandler;

import java.util.Arrays;
import java.util.List;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

import static org.assertj.core.api.Assertions.assertThat;
import static test.referential.TestReferential.APR_17_1991_AT_1100AM_CET;

class EventsProducedByCommandBusDispatcherTest {

    @Test
    public void it_should_handle_events_after_command_dispatching() throws InterruptedException {
        // Given
        var anEvent = new AnEvent(CommandId.create(), APR_17_1991_AT_1100AM_CET);
        var anEventHandler = new AnEventHandler();
        var anotherEventHandler = new AThirdEventHandler();
        var executorService = Executors.newSingleThreadExecutor();

        var command = new ACommand();
        List<Event> eventList = Arrays.asList(anEvent);
        var commandHandler = new CommandHandlerReturningAnEvent(eventList);

        var commandDispatcher = new CommandDispatcher(Arrays.asList(commandHandler));
        var eventDispatcher = new EventDispatcher(Arrays.asList(anEventHandler, anotherEventHandler));
        var eventDispatcherAfterCommandBusMiddleware = new EventsProducedByCommandBusDispatcher(eventDispatcher, commandDispatcher, executorService);

        // When
        var events = eventDispatcherAfterCommandBusMiddleware.dispatch(command);
        executorService.shutdown();
        executorService.awaitTermination(1, TimeUnit.MINUTES);

        // Then
        assertThat(anEventHandler.hasHandled(anEvent)).isTrue();
        assertThat(anotherEventHandler.hasHandled(anEvent)).isTrue();
        assertThat(events).containsAll(eventList);
    }


}