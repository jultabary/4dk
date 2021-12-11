# 4dk-security
The security project depends from the core one. It completes core with security and authorization concept by adding classes `Permission`, `Role`, `SecuredCommand` and `SecuredQuery`.

## Role
A `Role` is associated to a collection of `Permission`. This class is a projection of the real `Role` defined in your identity server. In your identity service, role is associated to user. Here, it makes the link between user roles and permission defined in the software. `RoleReadRepository` interface has to be implemented and allows to query `Role` and `Permission`. `RoleWrittePermission` interface has to be implemented and allows to set all `Role` and `Permission` needed in your software. <br />
Beware no association between `Role` and a user will be made in this framework.

## Permission
A `Permission` describe an authorization needed to access to a feature. You can attached a `Permission` with a `SecuredCommandHandler` or a `SecuredQueryHandler`.

## Authorization
A `Authorization` describes a valid or an unvalid access to a feature. It is returned by `AuthorizedStrategy` interface. (`Authorization isAuthorized(Permission expectedPermission, List<Role> givenRoles);`) <br />
This project proposes the following `AuthoizedStrategy`:
* `RoleBasedStrategy`: RBAC concept. It takes `Role`s and checks if expected `Permission` is contained in these `Role`s.
* `NoSecurityStrategy`: It will always returned true.

## Command
This project implements an extension of `Command` with `SecuredCommand`. A `SecuredCommand` can only trigger a `SecuredCommandHandler`.<br/>

### CommandBus
This project proposes the following commandbus:
*  `SecuredCommandDispatcher`: it overrides `CommandDispatcher` class. its constructor takes in entry an `AuthorizedStrategy` and a list of `CommandHandler`. This bus dispatches `Command` or `SecuredCommand` to correct `CommandHandler`. For a `SecuredCommand`, it checks if `Permission` associated to its handlers is contained in command `Roles`. Depends of `AuthorizedStrategy` it will authorized or not the access to the handler.

## Query
This project implements an extension of `Query` with `SecuredQuery`. A `SecuredQuery` can only trigger a `SecuredQueryHandler`.<br/>

### QueryBus
This project proposes the following query:
*  `SecuredQueryDispatcher`: it overrides `QueryDispatcher` class. its constructor takes in entry an `AuthorizedStrategy` and a list of `QueryHandler`. This bus dispatches `Query` or `SecuredQuery` to correct `QuerydHandler`. For a `SecuredQuery`, it checks if `Permission` associated to its handlers is contained in query `Role`s. Depends of `AuthorizedStrategy` it will authorized or not the access to the handler.