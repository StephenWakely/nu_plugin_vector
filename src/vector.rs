use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;

use crate::query::{component_query, components};

pub struct Vector;

impl Vector {
    pub fn query(&self, call: &EvaluatedCall, _input: &Value) -> Result<Value, LabeledError> {
        run_query(call)
    }
}

pub fn run_query(call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let address = call
        .get_flag("address")?
        .unwrap_or_else(|| "http://localhost:8686".to_string());
    let vars = component_query::Variables {};

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        match components(address, vars).await {
            Ok(result) => {
                let vals = result
                    .components
                    .nodes
                    .iter()
                    .map(|node| Value::String {
                        val: node.component_id.clone(),
                        span: call.head,
                    })
                    .collect();
                Ok(Value::List {
                    vals,
                    span: call.head,
                })
            }
            Err(err) => Err(LabeledError {
                label: "Error".to_string(),
                msg: err.to_string(),
                span: None,
            }),
        }
    })
}
