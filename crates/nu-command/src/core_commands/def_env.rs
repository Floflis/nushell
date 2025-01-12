use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Category, Example, PipelineData, Signature, Span, SyntaxShape, Value};

#[derive(Clone)]
pub struct DefEnv;

impl Command for DefEnv {
    fn name(&self) -> &str {
        "def-env"
    }

    fn usage(&self) -> &str {
        "Define a custom command, which participates in the caller environment"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("def-env")
            .required("def_name", SyntaxShape::String, "definition name")
            .required("params", SyntaxShape::Signature, "parameters")
            .required(
                "block",
                SyntaxShape::Block(Some(vec![])),
                "body of the definition",
            )
            .category(Category::Core)
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        Ok(PipelineData::new(call.head))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Set environment variable by call a custom command",
            example: r#"def-env foo [] { let-env BAR = "BAZ" }; foo; $env.BAR"#,
            result: Some(Value::String {
                val: "BAZ".to_string(),
                span: Span::test_data(),
            }),
        }]
    }
}
