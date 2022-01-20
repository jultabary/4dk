use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;
use dddk_core::dddk::aliases::Responses;
use dddk_core::dddk::errors::NoQueryHandlerRegisterForGivenQuery;
use dddk_core::dddk::query::query::Query;
use dddk_core::dddk::query::query_bus::QueryBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use crate::dddk::security::authorized_strategy::AuthorizedStrategy;
use crate::dddk::security::errors::{Forbidden, TryToExecuteASecuredQueryHandlerWithAnUnSecuredQuery};
use crate::dddk::security::permission::Permission;
use crate::dddk::security::query::secured_query::SecuredQuery;
use crate::dddk::security::query::secured_query_handler::SecuredQueryHandler;

pub struct SecuredQueryDispatcher {
    secured_handler_expected_permission: HashMap<TypeId, Permission>,
    query_handlers: HashMap<TypeId, Box<dyn QueryHandlerInBus>>,
    authorized_strategy: Rc<dyn AuthorizedStrategy>,
}

impl SecuredQueryDispatcher {
    pub fn new(given_query_handlers: Vec<Box<dyn QueryHandlerInBus>>, authorized_strategy: Rc<dyn AuthorizedStrategy>) -> SecuredQueryDispatcher {
        let mut secured_handler_expected_permission = HashMap::new();
        let mut query_handlers = HashMap::new() as HashMap<TypeId, Box<dyn QueryHandlerInBus>>;
        given_query_handlers.into_iter().for_each(|query_handler| {
            let mut associated_query_type_id = query_handler.get_associated_query_from_bus();
            if let Some(secured_query_handler) = query_handler.as_any().downcast_ref::<SecuredQueryHandler>() {
                associated_query_type_id = secured_query_handler.get_associated_query_from_bus();
                let associated_permission = secured_query_handler.get_associated_permission();
                secured_handler_expected_permission.insert(associated_query_type_id.clone(), associated_permission);
            }
            if let Some(_) = query_handlers.get(&associated_query_type_id) {
                panic!("A queryHandler has already been registered for this query");
            }
            query_handlers.insert(associated_query_type_id, query_handler);
        });
        SecuredQueryDispatcher {
            secured_handler_expected_permission,
            query_handlers,
            authorized_strategy,
        }
    }

    pub fn is_query_handler_restricted(&self, query_type_id: TypeId) -> bool {
        self.secured_handler_expected_permission.get(&query_type_id).is_some()
    }

    pub fn get_query_handler_associated_to_query(&self, query_type_id: TypeId) -> Option<&Box<dyn QueryHandlerInBus>> {
        self.query_handlers.get(&query_type_id)
    }

    fn get_query_handler_from_secured_query(&self, secured_query: &SecuredQuery) -> Result<&dyn QueryHandlerInBus, NoQueryHandlerRegisterForGivenQuery> {
        self.get_query_handler_from_query(secured_query.get_query())
    }

    fn get_query_handler_from_query(&self, query: &dyn Query) -> Result<&dyn QueryHandlerInBus, NoQueryHandlerRegisterForGivenQuery> {
        let query_type_id = query.as_any().type_id();
        if let Some(query_handler) = self.query_handlers.get(&query_type_id) {
            return Ok(query_handler.as_ref());
        }
        Err(NoQueryHandlerRegisterForGivenQuery {})
    }

    fn get_handler_expected_permission(&self, secured_query: &SecuredQuery) -> Permission {
        let query_type_id = secured_query.get_query().as_any().type_id();
        self.secured_handler_expected_permission.get(&query_type_id).unwrap().clone()
    }
}

impl QueryBus for SecuredQueryDispatcher {
    fn dispatch(&self, query: &dyn Query) -> Responses {
        return if let Some(secured_query) = query.as_any().downcast_ref::<SecuredQuery>() {
            let query_handler_result = self.get_query_handler_from_secured_query(secured_query);
            if query_handler_result.is_err() {
                return Err(Box::new(query_handler_result.err().unwrap()));
            }
            let query_handler = query_handler_result.unwrap();
            let permission = self.get_handler_expected_permission(secured_query);
            let authorization = self.authorized_strategy.is_authorized(
                permission,
                secured_query.get_roles_names(),
            );
            if authorization.is_authorized() {
                return query_handler.handle_from_bus(secured_query.get_query());
            }
            return Err(Box::new(Forbidden {}));
        } else {
            let query_handler_result = self.get_query_handler_from_query(query);
            if query_handler_result.is_err() {
                return Err(Box::new(query_handler_result.err().unwrap()));
            }
            let query_handler = query_handler_result.unwrap();
            if let Some(_) = query_handler.as_any().downcast_ref::<SecuredQueryHandler>() {
                return Err(Box::new(TryToExecuteASecuredQueryHandlerWithAnUnSecuredQuery {}));
            }
            query_handler.handle_from_bus(query)
        };
    }
}
