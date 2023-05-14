use crate::Vector;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, SyntaxShape, Value};

impl Plugin for Vector {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("vec")
            .usage("Queries Vectors components")
            .named(
                "address",
                SyntaxShape::String,
                "Optional address",
                Some('a'),
            )
            .plugin_examples(vec![PluginExample {
                example: "vec".into(),
                description: "Query the running vectors components".into(),
                result: None,
            }])
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "vec" => self.query(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
