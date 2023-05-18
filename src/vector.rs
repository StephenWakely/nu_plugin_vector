use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};

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
                    .map(|node| node_to_record(node, call.head))
                    .collect();
                // Value::as_block(&self)
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

fn node_to_record(node: &component_query::ComponentQueryComponentsNodes, span: Span) -> Value {
    let cols = vec![
        "component_id".to_string(),
        "component_type".to_string(),
        "events".to_string(),
    ];

    let event = match &node.on {
        component_query::ComponentQueryComponentsNodesOn::Sink(sink) => {
            sink.metrics
                .sent_events_total
                .as_ref()
                .unwrap()
                .sent_events_total
        }
        component_query::ComponentQueryComponentsNodesOn::Source(source) => {
            source
                .metrics
                .received_events_total
                .as_ref()
                .unwrap()
                .received_events_total
        }
        component_query::ComponentQueryComponentsNodesOn::Transform(transform) => {
            transform
                .metrics
                .processed_events_total
                .as_ref()
                .unwrap()
                .processed_events_total
        }
    };

    Value::Record {
        cols,
        vals: vec![
            Value::String {
                val: node.component_id.clone(),
                span,
            },
            Value::String {
                val: node.component_type.clone(),
                span,
            },
            Value::Float { val: event, span },
        ],
        span,
    }
}
