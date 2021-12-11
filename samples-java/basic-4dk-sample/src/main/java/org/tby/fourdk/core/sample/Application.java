package org.tby.fourdk.core.sample;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.command.CommandHandler;
import org.tby.fourdk.core.command.bus.CommandBus;
import org.tby.fourdk.core.command.bus.CommandDispatcher;
import org.tby.fourdk.core.command.bus.CommandLoggingMiddleware;
import org.tby.fourdk.core.command.bus.EventsProducedByCommandBusDispatcher;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.core.event.EventHandler;
import org.tby.fourdk.core.event.bus.EventDispatcher;
import org.tby.fourdk.core.sample.domain.GateRepository;
import org.tby.fourdk.core.sample.domain.ParkingRepository;
import org.tby.fourdk.core.sample.domain.ScreenRepository;
import org.tby.fourdk.core.sample.usecases.commands.ParkACarCommandHandler;
import org.tby.fourdk.core.sample.usecases.events.OpenGateWhenANewCarEnteredEventHandler;
import org.tby.fourdk.core.sample.usecases.events.RefreshScreenWhenAvailableSpotHasChangedEventHandler;

import java.util.Arrays;
import java.util.List;
import java.util.concurrent.Executors;

import static org.tby.fourdk.core.event.EventHandlerLoggerFactory.addLoggerToEventHandlers;

public class Application implements CommandBus {

    static List<CommandHandler> injectCommandHandlers(ParkingRepository parkingRepository) {
        var parkACarCommandHandler = new ParkACarCommandHandler(parkingRepository);
        return Arrays.asList(parkACarCommandHandler);
    }

    static List<EventHandler> injectEventHandlers(ParkingRepository parkingRepository, GateRepository gateRepository, ScreenRepository screenRepository) {
        var openGateWhenANewCarEnteredEventHandler = new OpenGateWhenANewCarEnteredEventHandler(parkingRepository, gateRepository);
        var refreshScreenWhenAvailableSpotHasChangedEventHandler = new RefreshScreenWhenAvailableSpotHasChangedEventHandler(screenRepository, parkingRepository);
        List<EventHandler> eventHandlers = Arrays.asList(openGateWhenANewCarEnteredEventHandler, refreshScreenWhenAvailableSpotHasChangedEventHandler);
        return addLoggerToEventHandlers(eventHandlers);
    }

    private CommandBus commandBus;

    public Application(ParkingRepository parkingRepository,
                       ScreenRepository screenRepository,
                       GateRepository gateRepository) {
        var commandDispatcher = new CommandDispatcher(injectCommandHandlers(parkingRepository));
        var eventDispatcher = new EventDispatcher(injectEventHandlers(parkingRepository, gateRepository, screenRepository));
        var loggingMiddleware = new CommandLoggingMiddleware(commandDispatcher);

        this.commandBus = new EventsProducedByCommandBusDispatcher(eventDispatcher, loggingMiddleware, Executors.newSingleThreadExecutor());
    }


    @Override
    public List<Event> dispatch(Command command) {
        return this.commandBus.dispatch(command);
    }
}
