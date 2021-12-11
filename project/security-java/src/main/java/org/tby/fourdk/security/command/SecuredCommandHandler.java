package org.tby.fourdk.security.command;

import org.tby.fourdk.core.command.CommandHandler;
import org.tby.fourdk.security.Permission;

public interface SecuredCommandHandler<C extends SecuredCommand> extends CommandHandler<C> {

    Permission getAssociatedPermission();
}
