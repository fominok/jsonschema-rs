use super::CompilationResult;
use super::{Validate, Validators};
use crate::context::CompilationContext;
use crate::error::{no_error, CompilationError, ErrorIterator};
use crate::validator::compile_validators;
use crate::JSONSchema;
use regex::Regex;
use serde_json::{Map, Value};

pub struct PatternPropertiesValidator {
    patterns: Vec<(Regex, Validators)>,
}

impl PatternPropertiesValidator {
    pub(crate) fn compile(properties: &Value, context: &CompilationContext) -> CompilationResult {
        match properties.as_object() {
            Some(map) => {
                let mut patterns = Vec::with_capacity(map.len());
                for (pattern, subschema) in map {
                    patterns.push((
                        Regex::new(pattern)?,
                        compile_validators(subschema, context)?,
                    ));
                }
                Ok(Box::new(PatternPropertiesValidator { patterns }))
            }
            None => Err(CompilationError::SchemaError),
        }
    }
}

impl Validate for PatternPropertiesValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Object(item) = instance {
            let errors: Vec<_> = self
                .patterns
                .iter()
                .flat_map(move |(re, validators)| {
                    item.iter()
                        .filter(move |(key, _)| re.is_match(key))
                        .flat_map(move |(_key, value)| {
                            validators
                                .iter()
                                .flat_map(move |validator| validator.validate(schema, value))
                        })
                })
                .collect();
            return Box::new(errors.into_iter());
        }
        no_error()
    }
    fn name(&self) -> String {
        format!("<pattern properties: {:?}>", self.patterns)
    }
}

pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &CompilationContext,
) -> Option<CompilationResult> {
    Some(PatternPropertiesValidator::compile(schema, context))
}
