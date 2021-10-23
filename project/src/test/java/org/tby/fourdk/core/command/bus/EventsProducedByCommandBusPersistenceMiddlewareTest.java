package org.tby.fourdk.core.command.bus;

import org.junit.jupiter.api.Test;
import org.tby.fourdk.core.command.CommandId;
import org.tby.fourdk.core.event.Event;
import test.fake.command.ACommand;
import test.fake.command.CommandHandlerReturningAnEvent;
import test.fake.event.AnEvent;
import test.fake.event.FakeEventRepository;

import java.util.Arrays;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static test.referential.TestReferential.APR_17_1991_AT_1100AM_CET;

class EventsProducedByCommandBusPersistenceMiddlewareTest {

    @Test
    public void it_should_handle_events_after_command_dispatching() throws InterruptedException {
        // Given
        var anEvent = new AnEvent(CommandId.create(), APR_17_1991_AT_1100AM_CET);
        var eventRepository = new FakeEventRepository();
        var command = new ACommand();
        List<Event> eventList = Arrays.asList(anEvent);
        var commandHandler = new CommandHandlerReturningAnEvent(eventList);

        var commandDispatcher = new CommandDispatcher(Arrays.asList(commandHandler));
        var bus = new EventsProducedByCommandBusPersistenceMiddleware(commandDispatcher, eventRepository);

        // When
        bus.dispatch(command);

        // Then
        assertThat(eventRepository.isEventPersisted(anEvent)).isTrue();
    }


}