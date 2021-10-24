package test.fake.command;

import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.command.SecuredCommand;
import org.tby.fourdk.security.userinfo.UserId;

import java.util.List;

public class ASecuredCommand extends SecuredCommand {

    public ASecuredCommand(List<Role> roles, UserId userId) {
        super(roles, userId);
    }
}
