package org.tby.fourdk.core.externalevent;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.tby.fourdk.core.command.Command;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class ExternalEventDispatcher implements ExternalEventBus {

    private static final Logger LOGGER = LoggerFactory.getLogger(ExternalEventDispatcher.class);

    private Map<Class, PolicyHandler> policyHandlersRegistered;

    public ExternalEventDispatcher(List<PolicyHandler> policyHandlers) {
        this.policyHandlersRegistered = new HashMap<>();
        registerPolicyHandlers(policyHandlers);
    }


    @Override
    public List<Command> dispatch(ExternalEvent event) {
        if (isEventHandled(event.getClass())) {
            var policyHandler = this.policyHandlersRegistered.get(event.getClass());
            return policyHandler.handle(event);
        } else {
            throw new UnhandledExternalEventException("External Event of type [" + event.getClass().getSimpleName() + "] has no associated handler.");
        }
    }

    private boolean isEventHandled(Class<? extends ExternalEvent> eventClass) {
        return this.policyHandlersRegistered.containsKey(eventClass);
    }

    private void registerPolicyHandlers(List<PolicyHandler> policyHandlers) {
        for (PolicyHandler policyHandler : policyHandlers) {
            var handlerType = policyHandler.getClass().getSimpleName();
            var eventType = policyHandler.getClass().getSimpleName();
            this.policyHandlersRegistered.put(policyHandler.getEventType(), policyHandler);
            LOGGER.info("Register PolicyHandler [{}] for event [{}]", handlerType, eventType);
        }
    }
}
