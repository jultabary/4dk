package org.tby.fourdk.security.command;

import org.tby.fourdk.core.command.Command;
import org.tby.fourdk.core.command.CommandHandler;
import org.tby.fourdk.core.command.bus.CommandDispatcher;
import org.tby.fourdk.core.event.Event;
import org.tby.fourdk.security.Authorization;
import org.tby.fourdk.security.AuthorizedStrategy;
import org.tby.fourdk.security.ForbiddenAccessException;
import org.tby.fourdk.security.Role;

import java.util.ArrayList;
import java.util.List;

public class SecuredCommandDispatcher extends CommandDispatcher {

    private AuthorizedStrategy authorizedStrategy;

    public SecuredCommandDispatcher(AuthorizedStrategy authorizedStrategy, List<CommandHandler> commandHandlers) {
        super(commandHandlers);
        this.authorizedStrategy = authorizedStrategy;
    }

    @Override
    public List<Event> dispatch(Command command) {
        var commandHandler = this.commandHandlers.get(command.getClass());
        List<Role> roles = new ArrayList<Role>();
        if (command instanceof SecuredCommand) {
            roles = ((SecuredCommand) command).userRoles != null ? ((SecuredCommand) command).userRoles : roles;
        }
        if (commandHandler instanceof SecuredCommandHandler) {
            Authorization authorization = authorizedStrategy.isAuthorized(((SecuredCommandHandler) commandHandler).getAssociatedPermission(), roles);
            if (!authorization.isAuthorized) {
                throw new ForbiddenAccessException("Access to [" + commandHandler.getClass().getSimpleName() + "] is forbidden for the given roles");
            }
            if (command instanceof SecuredCommand) {
                ((SecuredCommand) command).setPermissions(new ArrayList<>(authorization.permissions));
            }
        }
        return super.dispatch(command);
    }

}
