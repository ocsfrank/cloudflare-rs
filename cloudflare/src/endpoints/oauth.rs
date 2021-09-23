use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::ApiResult;

#[derive(Debug)]
pub struct RevokeConsentSession {
    pub params: RevokeConsentSessionParams,
}

impl<'a> Endpoint<RevokeConsentID, RevokeConsentSessionParams, ()> for RevokeConsentSession {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        "user/iam/oauth_consent_sessions".to_string()
    }

    fn query(&self) -> Option<RevokeConsentSessionParams> {
        self.params.clone()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct RevokeConsentSessionParams {
    /// client_id to revoke consent session from
    pub client_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RevokeConsentID {
    pub client_id: String,
}