//! API request and response types for MetaSSR API routes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Headers type alias for convenience.
pub type Headers = HashMap<String, String>;

/// Represents an incoming API request passed to handler functions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiRequest {
    /// The URL path of the request.
    pub url: String,
    /// HTTP headers as key-value pairs.
    pub headers: Headers,
    /// HTTP method (GET, POST, etc.).
    pub method: String,
    /// Request body (if any).
    pub body: Option<String>,
    /// URL path parameters (e.g., from dynamic routes).
    pub params: HashMap<String, String>,
    /// Query string parameters.
    pub query: HashMap<String, String>,
}

/// Represents an API response returned by handler functions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response headers (optional).
    #[serde(default)]
    pub headers: Headers,
    /// Response body as JSON value.
    pub body: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LanguageId {
    Node,
    Py,
    Rb,
    Ts
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum ValueId {
    MetacallBool     = 0,
    MetacallChar     = 1,
    MetacallShort    = 2,
    MetacallInt      = 3,
    MetacallLong     = 4,
    MetacallFloat    = 5,
    MetacallDouble   = 6,
    MetacallString   = 7,
    MetacallBuffer   = 8,
    MetacallArray    = 9,
    MetacallMap      = 10,
    MetacallPtr      = 11,
    MetacallFuture   = 12,
    MetacallFunction = 13,
    MetacallNull     = 14,
    MetacallClass    = 15,
    MetacallObject   = 16,
    MetacallSize     = 17,
    MetacallInvalid  = 18,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaCallJSON {
    pub language_id: LanguageId,
    pub path: String,
    pub scripts: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub path: String,
    pub jsons: Vec<MetaCallJSON>,
    pub runners: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaCallType {
    pub name: String,
    pub id: ValueId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Return {
    #[serde(rename = "type")]
    pub type_info: MetaCallType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Argument {
    pub name: String,
    #[serde(rename = "type")]
    pub type_info: MetaCallType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    pub ret: Return,
    pub args: Vec<Argument>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Func {
    pub name: String,
    pub signature: Signature,
    #[serde(rename = "async")]
    pub async_func: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scope {
    pub name: String,
    pub funcs: Vec<Func>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Handle {
    pub name: String,
    pub scope: Scope
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentStatus {
    Create,
    Ready,
    Fail
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deployment {
    pub status: DeploymentStatus,
    pub prefix: String,
    pub suffix: String,
    pub version: String,
    pub packages: HashMap<LanguageId, Vec<Handle>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvokePayload {
    pub id: String,
    pub name: String,
    pub args: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvokeResultPayload {
    pub id: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WorkerMessage {
    #[serde(rename = "LoadFunctions")]
    Load(Resource),

    #[serde(rename = "GetApplicationMetadata")]
    MetaData(Deployment),

    #[serde(rename = "CallFunction")]
    Invoke(InvokePayload),

    #[serde(rename = "FunctionInvokeResult")]
    InvokeResult(InvokeResultPayload),
}