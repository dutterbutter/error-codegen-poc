pub mod config;
pub mod error;
pub mod files;

use std::path::PathBuf;

use config::RustBackendConfig;
use error::GenerationError;

use crate::codegen::printer::PrettyPrinter;
use crate::model::error::ModelError;
use crate::model::structure::ComponentDescription;
use crate::model::structure::DomainDescription;
use crate::model::structure::ErrorDescription;
use crate::model::structure::FieldDescription;
use crate::model::structure::FullyQualifiedTargetLanguageType;
use crate::model::structure::Model;

use super::Backend;
use super::File;

pub struct RustBackend {
    model: Model,
}

impl Backend<RustBackendConfig> for RustBackend {
    type Error = GenerationError;

    fn get_name() -> &'static str {
        "rust"
    }

    fn generate(&mut self, _config: &RustBackendConfig) -> Result<Vec<File>, Self::Error> {
        let vec = vec![
            self.generate_file_error_definitions()?,
            self.generate_file_error_domains()?,
            self.generate_file_error_mod()?,
            self.generate_file_identifier()?,
            self.generate_file_kind()?,
            self.generate_file_lib()?,
            self.generate_file_packed()?,
            self.generate_file_serialized()?,
            self.generate_file_untyped()?,
            File {
                relative_path: PathBuf::from("Cargo.toml"),
                content: r#"
[package]
name = "zksync_error"
version = "0.1.0"
edition = "2021"
[lib]
[dependencies]
serde = "1.0.213"
serde_json = "1.0.132"
strum = "0.26.3"
strum_macros = "0.26.4"
typify = "0.2.0"
"#
                .into(),
            },
        ];
        Ok(vec)
    }

    fn get_language_name() -> &'static str {
        "rust"
    }
}

impl RustBackend {
    pub fn new(model: &Model) -> Self {
        Self {
            model: model.clone(),
        }
    }

    fn preamble(file: &mut PrettyPrinter) {
        let string = r#"
//!
//! AUTOGENERATED BASED ON A SET OF JSON FILES, DO NOT EDIT MANUALLY
//!
"#;
        file.push_str(string);
    }

    fn type_as_rust(typ: &FullyQualifiedTargetLanguageType) -> String {
        let FullyQualifiedTargetLanguageType { name, path } = typ;
        if path.is_empty() {
            name.to_string()
        } else {
            format!("{path}.{name}")
        }
    }

    fn get_rust_type(&self, name: &str) -> Result<String, GenerationError> {
        let typ = self.model.get_type(Self::get_language_name(), name)?;
        Ok(Self::type_as_rust(typ))
    }

    fn error_field(&self, field: &FieldDescription) -> Result<String, GenerationError> {
        let FieldDescription { name, r#type } = field;
        let rust_type = self.get_rust_type(r#type)?;

        Ok(format!("{name} : {rust_type},"))
    }

    fn error_kind(&self, error: &ErrorDescription) -> Result<String, GenerationError> {
        let ErrorDescription {
            name, code, fields, ..
        } = error;
        let mut result = PrettyPrinter::new(1024);
        result.push_line(&format!("{name} {{ "));
        result.indentation.increase();
        for field in fields {
            result.push_line(&self.error_field(field)?);
        }
        result.indentation.decrease();

        result.push_line(&format!("}} = {code}, "));
        Ok(result.get_buffer())
    }

    fn error_kind_match(
        &self,
        component: &ComponentDescription,
        error: &ErrorDescription,
    ) -> Result<String, GenerationError> {
        let component_name = Self::component_type_name(component)?;
        let ErrorDescription { name, fields, .. } = error;
        let mut result = PrettyPrinter::default();

        result.push_line(&format!("{component_name}::{name} {{ "));

        result.indentation.increase();
        for field in fields {
            let field_name = &field.name;
            result.push_line(&format!("{field_name},"));
        }
        result.indentation.decrease();

        result.push_line("}");
        Ok(result.get_buffer())
    }

    fn component_type_name(component: &ComponentDescription) -> Result<String, GenerationError> {
        let name = component
            .meta
            .bindings
            .get(Self::get_language_name())
            .ok_or(ModelError::UnmappedName(component.meta.name.clone()))?;

        Ok(name.to_string())
    }
    fn component_code_type_name(
        component: &ComponentDescription,
    ) -> Result<String, GenerationError> {
        let name = component
            .meta
            .bindings
            .get(Self::get_language_name())
            .ok_or(ModelError::UnmappedName(component.meta.name.clone()))?;

        Ok(format!("{name}Code"))
    }

    fn domain_type_name(domain: &DomainDescription) -> Result<String, GenerationError> {
        let name = domain
            .meta
            .bindings
            .get(Self::get_language_name())
            .ok_or(ModelError::UnmappedName(domain.meta.name.clone()))?;

        Ok(name.to_string())
    }

    fn domain_code_type_name(domain: &DomainDescription) -> Result<String, GenerationError> {
        let name = domain
            .meta
            .bindings
            .get(Self::get_language_name())
            .ok_or(ModelError::UnmappedName(domain.meta.name.clone()))?;

        Ok(format!("{name}Code"))
    }
}
