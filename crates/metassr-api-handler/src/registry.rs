use anyhow::{anyhow, Result};
use metacall::load;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use tracing::{debug, info, warn};

use crate::types::{
    Deployment, DeploymentStatus, Handle, InvokePayload, InvokeResultPayload, LanguageId,
    MetaCallJSON, Resource, Scope,
};
pub struct FunctionRegistry {
    loaded_scripts: HashSet<String>,
    loaded_by_language: HashMap<LanguageId, Vec<String>>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            loaded_scripts: HashSet::new(),
            loaded_by_language: HashMap::new(),
        }
    }

    pub fn handle_load(&mut self, resource: Resource) -> Result<Deployment> {
        info!(
            "Loading resource '{}' from path: {}",
            resource.id, resource.path
        );

        let base_path = Path::new(&resource.path);

        if resource.jsons.is_empty() {
            warn!(
                "No MetaCallJSON configs in resource '{}', attempting direct script scan",
                resource.id
            );
            self.load_scripts_from_dir(base_path)?;
        } else {
            for json in &resource.jsons {
                self.load_from_json(base_path, json)?;
            }
        }

        let deployment = self.build_deployment(&resource);
        info!(
            "Resource '{}' loaded. Status: {:?}",
            resource.id, deployment.status
        );

        Ok(deployment)
    }

    fn load_from_json(&mut self, base_path: &Path, json: &MetaCallJSON) -> Result<()> {
        let tag = language_id_to_tag(&json.language_id);

        for script in &json.scripts {
            let script_path = if Path::new(script).is_absolute() {
                script.clone()
            } else {
                base_path
                    .join(&json.path)
                    .join(script)
                    .to_string_lossy()
                    .to_string()
            };

            self.load_single_script(&script_path, tag, json.language_id.clone())?;
        }

        Ok(())
    }

    fn load_scripts_from_dir(&mut self, dir: &Path) -> Result<()> {
        for entry in fs::read_dir(dir)?.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let (tag, lang) = match ext {
                    "js" => (load::Tag::NodeJS, LanguageId::Node),
                    "py" => (load::Tag::Python, LanguageId::Py),
                    "rb" => (load::Tag::Ruby, LanguageId::Rb),
                    "ts" => (load::Tag::TypeScript, LanguageId::Ts),
                    _ => continue,
                };
                let path_str = path.to_string_lossy().to_string();
                self.load_single_script(&path_str, tag, lang)?;
            }
        }
        Ok(())
    }

    fn load_single_script(
        &mut self,
        script_path: &str,
        tag: load::Tag,
        language: LanguageId,
    ) -> Result<()> {
        if self.loaded_scripts.contains(script_path) {
            debug!("Script already loaded, skipping: {}", script_path);
            return Ok(());
        }
        load::from_file(tag, &[script_path], None)
            .map_err(|e| anyhow!("MetaCall failed to load '{}': {:?}", script_path, e))?;

        info!("Loaded script: {}", script_path);

        self.loaded_scripts.insert(script_path.to_string());
        self.loaded_by_language
            .entry(language)
            .or_default()
            .push(script_path.to_string());

        Ok(())
    }

    pub fn invoke(&self, payload: InvokePayload) -> InvokeResultPayload {
        use metacall::{
            match_metacall_value, metacall_untyped, MetaCallError, MetaCallNull, MetaCallValue,
        };

        debug!(
            "Invoking function '{}' with {} arg(s) (id: {})",
            payload.name,
            payload.args.len(),
            payload.id
        );

        let args: Vec<Box<dyn MetaCallValue>> = payload
            .args
            .iter()
            .map(|v| -> Box<dyn MetaCallValue> {
                match v {
                    serde_json::Value::Bool(b) => Box::new(*b),
                    serde_json::Value::Number(n) => Box::new(n.as_f64().unwrap_or(0.0)),
                    serde_json::Value::String(s) => Box::new(s.clone()),
                    serde_json::Value::Null => Box::new(MetaCallNull()),
                    other => Box::new(other.to_string()),
                }
            })
            .collect();

        let result = match metacall_untyped(&payload.name, args) {
            Err(MetaCallError::FunctionNotFound) => {
                serde_json::json!({
                    "error": format!("Function '{}' not found in MetaCall context", payload.name)
                })
            }
            Err(e) => {
                warn!("Invocation of '{}' failed: {:?}", payload.name, e);
                serde_json::json!({ "error": format!("{:?}", e) })
            }

            Ok(val) => match_metacall_value!(val, {
                b:    bool        => serde_json::Value::Bool(b),
                n:    i16         => serde_json::json!(n),
                n:    i32         => serde_json::json!(n),
                n:    i64         => serde_json::json!(n),
                n:    f32         => serde_json::json!(n),
                n:    f64         => if n.fract() == 0.0 && n.abs() < i64::MAX as f64 {
                                         serde_json::json!(n as i64)
                                     } else {
                                         serde_json::json!(n)
                                     },
                s:    String      => serde_json::from_str(&s)
                                         .unwrap_or(serde_json::Value::String(s)),
                null: MetaCallNull => serde_json::Value::Null,
                _ => {
                    debug!("Unhandled MetaCall return type for '{}'", payload.name);
                    serde_json::Value::Null
                }
            }),
        };

        InvokeResultPayload {
            id: payload.id,
            result,
        }
    }

    fn build_deployment(&self, resource: &Resource) -> Deployment {
        let mut packages = HashMap::new();

        for (lang, scripts) in &self.loaded_by_language {
            let handles: Vec<Handle> = scripts
                .iter()
                .map(|script_path| {
                    let file_name = Path::new(script_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(script_path)
                        .to_string();

                    Handle {
                        name: file_name,
                        scope: Scope {
                            name: "global".to_string(),
                            // skipped for the poc
                            funcs: vec![],
                        },
                    }
                })
                .collect();

            packages.insert(lang.clone(), handles);
        }

        Deployment {
            status: DeploymentStatus::Ready,
            prefix: hostname(),
            suffix: resource.id.clone(),
            version: "v1".to_string(),
            packages,
        }
    }
}

// Helpers
fn language_id_to_tag(lang: &LanguageId) -> load::Tag {
    match lang {
        LanguageId::Node => load::Tag::NodeJS,
        LanguageId::Py => load::Tag::Python,
        LanguageId::Rb => load::Tag::Ruby,
        LanguageId::Ts => load::Tag::TypeScript,
    }
}

fn hostname() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string())
}
