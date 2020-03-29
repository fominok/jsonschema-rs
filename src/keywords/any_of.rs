use super::CompilationResult;
use super::{Validate, Validators};
use crate::context::CompilationContext;
use crate::error::{no_error, CompilationError, ErrorIterator, ValidationError};
use crate::validator::compile_validators;
use crate::JSONSchema;
use serde_json::{Map, Value};

pub struct AnyOfValidator {
    schemas: Vec<Validators>,
}

impl AnyOfValidator {
    pub(crate) fn compile(schema: &Value, context: &CompilationContext) -> CompilationResult {
        match schema.as_array() {
            Some(items) => {
                let mut schemas = Vec::with_capacity(items.len());
                for item in items {
                    let validators = compile_validators(item, context)?;
                    schemas.push(validators)
                }
                Ok(Box::new(AnyOfValidator { schemas }))
            }
            None => Err(CompilationError::SchemaError),
        }
    }
}

impl Validate for AnyOfValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        for validators in self.schemas.iter() {
            if validators
                .iter()
                .all(|validator| validator.is_valid(schema, instance))
            {
                return no_error();
            }
        }
        ValidationError::any_of(instance.clone())
    }
    fn name(&self) -> String {
        format!("<any of: {:?}>", self.schemas)
    }
}

pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &CompilationContext,
) -> Option<CompilationResult> {
    Some(AnyOfValidator::compile(schema, context))
}
