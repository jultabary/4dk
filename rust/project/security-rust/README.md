# 4dk-security

The lib extends core behaviour with security features. <br/>
It implements the entites : `SecuredCommand`, `SecuredQuery`, `SecuredCommandHandler`, `SecuredQueryHandler`, `SecuredCommandDispatcher`, `SecuredQueryDispatcher`.

## Command

### SecuredCommand
Extends `Command` by decoration. SecuredCommand has to be constructed with user roles.

### SecuredCommandHandler
Extends `CommandHandler` by decoration. SecuredHandler is defined with a permission.

### SecuredCommandDispatcher
Replace the `CommandDispatcher` implementation from `core-rust`.

## Query

### SecuredQuery
Extends `Query` by decoration. SecuredQuery has to be constructed with user roles.

### SecuredQueryHandler
Extends `QueryHandler` by decoration. SecuredHandler is defined with a permission.


### SecuredQueryDispatcher
Replace the `QueryDispatcher` implementation from `core-rust`.

<br /><br />
You can find in samples different bus composition.
