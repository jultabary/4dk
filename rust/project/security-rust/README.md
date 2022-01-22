# 4dk-security

The lib extends core behaviour with security features. <br/>
It implements the entites : `SecuredCommand`, `SecuredQuery`, `SecuredCommandHandler`, `SecuredQueryHandler`, `SecuredCommandDispatcher`, `SecuredQueryDispatcher`.

## Command

### SecuredCommand
Extends `Command` by decoration. SecuredCommand has to be constructed with user roles.

### SecuredCommandHandler
Extends `CommandHandler` by decoration. SecuredHandler is defined with a permission.

### SecuredCommandDispatcher
Replace the `CommandDispatcher` implementation from `core-rust`.<br/>
When a command is dispatch to the `CommandBus`, it will replaces (with `RoleReadRepository` impl) user roles by its permissions. Then it will check if user command has the permission to call the handler.

## Query

### SecuredQuery
Extends `Query` by decoration. SecuredQuery has to be constructed with user roles.

### SecuredQueryHandler
Extends `QueryHandler` by decoration. SecuredHandler is defined with a permission.


### SecuredQueryDispatcher
Replace the `QueryDispatcher` implementation from `core-rust`.<br/>
When a query is dispatch to the `QueryBus`, it will replaces (with `RoleReadRepository` impl) user roles by its permissions. Then it will check if user query has the permission to call the handler.

<br /><br />
You can find in samples different bus composition.
