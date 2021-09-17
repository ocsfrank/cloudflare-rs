use crate::framework::endpoint::{Endpoint, Method};
use super::WorkersScript; 

/// List Secrets
/// Lists all secrets mappings for a given script
/// https://api.cloudflare.com/#worker-secrets-list-secrets
#[derive(Debug)]
pub struct ListScripts<'a> {
    pub account_id: &'a str,
}

impl<'a> Endpoint<Vec<WorkersScript>> for ListScripts<'a> {
    fn method(&self) -> Method {
        Method::Get
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts",
            self.account_id
        )
    }
}

