use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use tracing::{error, info};

use crate::{
    registry::FunctionRegistry,
    types::WorkerMessage,
};

#[derive(Clone)]
pub struct FaasState {
    pub registry: Arc<Mutex<FunctionRegistry>>,
}

impl FaasState {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(FunctionRegistry::new())),
        }
    }
}

impl Default for FaasState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn register_faas_routes(router: Router) -> (Router, FaasState) {
    let state = FaasState::new();

    let rpc_router = Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(state.clone());

    let router = router.merge(rpc_router);

    info!("FaaS RPC endpoint registered at POST /rpc");

    (router, state)
}

async fn rpc_handler(
    State(state): State<FaasState>,
    Json(message): Json<WorkerMessage>,
) -> impl IntoResponse {
    match message {
        WorkerMessage::Load(resource) => {
            let resource_id = resource.id.clone();
            info!("RPC /rpc received LoadFunctions for '{}'", resource_id);

            let result = {
                let mut registry = match state.registry.lock() {
                    Ok(r) => r,
                    Err(e) => {
                        error!("Registry mutex poisoned: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": "Internal registry lock failure"
                            })),
                        )
                            .into_response();
                    }
                };
                registry.handle_load(resource)
            };

            match result {
                Ok(deployment) => {
                    info!(
                        "LoadFunctions for '{}' succeeded. Sending MetaData.",
                        resource_id
                    );
                    
                    let response = WorkerMessage::MetaData(deployment);
                    (StatusCode::OK, Json(serde_json::to_value(response).unwrap())).into_response()
                }
                Err(e) => {
                    error!("LoadFunctions for '{}' failed: {}", resource_id, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": e.to_string() })),
                    )
                        .into_response()
                }
            }
        }

        WorkerMessage::Invoke(payload) => {
            let func_name = payload.name.clone();
            let invoke_id = payload.id.clone();
            info!("RPC /rpc received CallFunction '{}' (id: {})", func_name, invoke_id);

            let result_payload = {
                let registry = match state.registry.lock() {
                    Ok(r) => r,
                    Err(e) => {
                        error!("Registry mutex poisoned: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": "Internal registry lock failure"
                            })),
                        )
                            .into_response();
                    }
                };
                registry.invoke(payload)
            };

            info!(
                "CallFunction '{}' (id: {}) completed.",
                func_name, invoke_id
            );

            let response = WorkerMessage::InvokeResult(result_payload);
            (StatusCode::OK, Json(serde_json::to_value(response).unwrap())).into_response()
        }
        
        WorkerMessage::MetaData(_) | WorkerMessage::InvokeResult(_) => {
            error!("Received unexpected inbound-only message type on /rpc");
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "This message type is outbound-only"
                })),
            )
                .into_response()
        }
    }
}
