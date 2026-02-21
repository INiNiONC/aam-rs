use crate::commands::Command;

pub struct SchemaCommand;

impl Command for SchemaCommand {
    fn name(&self) -> &str {
        "schema"
    }

    fn execute(&self, _aaml: &mut crate::aaml::AAML, _args: &str) -> Result<(), crate::error::AamlError> {

        Ok(())
    }
}