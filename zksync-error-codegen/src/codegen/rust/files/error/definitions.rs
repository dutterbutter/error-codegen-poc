use crate::{
    codegen::{
        printer::PrettyPrinter,
        rust::{error::GenerationError, RustBackend},
        File,
    },
    model::ComponentDescription,
};

impl RustBackend {
    fn component_errors(
        &self,
        component: &ComponentDescription,
    ) -> Result<String, GenerationError> {
        let error_name = Self::component_type_name(component)?;
        let mut result = PrettyPrinter::default();
        result.push_line(&format!(
            r#"
#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name({error_name}Code))]
#[strum_discriminants(vis(pub))]
#[non_exhaustive]
pub enum {error_name} {{"#
        ));
        result.indent_increase();
        for error in &component.errors {
            result.push_block(&self.error_kind(&error)?);
        }
        result.push_line(&format!(
            r#"
}} // end of {error_name}
"#
        ));
        result.indent_decrease();

        result.push_line(&format!(
            r#"
impl CustomErrorMessage for {error_name} {{
    fn get_message(&self) -> String {{
        match self {{"#,
        ));
        result.indent_increase_by(3);

        for error in &component.errors {
            result.push_block(&self.error_kind_match(component, &error)?);
            let message = &error.message;
            result.push_line(&format!(" => {{ format!(\"{message}\") }},"));
        }
        for _ in 0..3 {
            result.indent_decrease();
            result.push_line("}");
        }
        Ok(result.get_buffer())
    }

    pub fn generate_error_definitions(&mut self) -> Result<File, GenerationError> {
        let mut gen = PrettyPrinter::default();

        Self::preamble(&mut gen);

        gen.push_str(
            r#"
use crate::error::CustomErrorMessage;
use strum_macros::EnumDiscriminants;
"#,
        );

        for component in self
            .model
            .domains
            .values()
            .flat_map(|domain| domain.components.values())
        {
            gen.push_str(&self.component_errors(component)?)
        }

        Ok(File {
            relative_path: vec!["error".into(), "definitions.rs".into()],
            content: gen.get_buffer(),
        })
    }
}
