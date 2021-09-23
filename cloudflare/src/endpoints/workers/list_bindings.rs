use crate::framework::endpoint::{Endpoint, Method};
use super::WorkersBinding; 

/// List Bindings
/// Lists all bindings for a given script
#[derive(Debug)]
pub struct ListBindings<'a> {
    pub account_id: &'a str,
    pub script_id: &'a str,
}

impl<'a> Endpoint<Vec<WorkersBinding>> for ListBindings<'a> {
    fn method(&self) -> Method {
        Method::Get
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/bindings",
            self.account_id,
            self.script_id
        )
    }
}
