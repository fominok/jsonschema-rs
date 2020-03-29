use super::CompilationResult;
use super::Validate;
use crate::context::CompilationContext;
use crate::error::{no_error, CompilationError, ErrorIterator};
use crate::{checks, JSONSchema};
use serde_json::{Map, Value};

pub struct FormatValidator {
    check: fn(&str) -> ErrorIterator,
}

impl<'a> FormatValidator {
    pub(crate) fn compile(check: fn(&str) -> ErrorIterator) -> CompilationResult {
        Ok(Box::new(FormatValidator { check }))
    }
}

impl Validate for FormatValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::String(item) = instance {
            return (self.check)(item);
        }
        no_error()
    }
    fn name(&self) -> String {
        // TODO. store name
        "<format: todo>".to_string()
    }
}

pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    match schema.as_str() {
        Some(format) => {
            let func = match format {
                "date" => checks::date,
                "date-time" => checks::datetime,
                "email" => checks::email,
                "hostname" => checks::hostname,
                "idn-email" => checks::email, // TODO. should have "idn-email" in the error message
                "idn-hostname" => checks::hostname,
                "ipv4" => checks::ipv4,
                "ipv6" => checks::ipv6,
                "iri" => checks::iri,
                "iri-reference" => checks::iri_reference,
                "json-pointer" => checks::json_pointer,
                "regex" => checks::regex,
                "relative-json-pointer" => checks::relative_json_pointer,
                "time" => checks::time,
                "uri" => checks::iri,
                "uri-reference" => checks::uri_reference,
                "uri-template" => checks::uri_template,
                _ => return None,
            };
            Some(FormatValidator::compile(func))
        }
        None => Some(Err(CompilationError::SchemaError)),
    }
}
