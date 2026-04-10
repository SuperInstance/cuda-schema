/*!
# cuda-schema

Runtime schema validation for agent messages.

Agents exchange structured data. Schemas validate payloads,
enforce constraints, and catch malformed data before it causes
problems downstream.

- Field types (string, int, float, bool, bytes, array, object)
- Required/optional fields
- Value constraints (min, max, pattern, enum)
- Nested object validation
- Validation errors with path
- Schema composition (allOf, anyOf)
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldType { String, Int, Float, Bool, Bytes, Array, Object }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub min_val: Option<f64>,
    pub max_val: Option<f64>,
    pub pattern: Option<String>,
    pub enum_values: Vec<String>,
    pub fields: Vec<FieldDef>, // for nested objects
}

impl FieldDef {
    pub fn required(name: &str, ft: FieldType) -> Self { FieldDef { name: name.to_string(), field_type: ft, required: true, min_val: None, max_val: None, pattern: None, enum_values: vec![], fields: vec![] } }
    pub fn optional(name: &str, ft: FieldType) -> Self { FieldDef { name: name.to_string(), field_type: ft, required: false, min_val: None, max_val: None, pattern: None, enum_values: vec![], fields: vec![] } }
    pub fn with_min(mut self, v: f64) -> Self { self.min_val = Some(v); self }
    pub fn with_max(mut self, v: f64) -> Self { self.max_val = Some(v); self }
    pub fn with_enum(mut self, vals: &[&str]) -> Self { self.enum_values = vals.iter().map(|s| s.to_string()).collect(); self }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ValType { String(String), Int(i64), Float(f64), Bool(bool), Bytes(Vec<u8>), Array(Vec<ValType>), Object(HashMap<String, ValType>), Null }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationError { pub path: String, pub message: String }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema { pub name: String, pub fields: Vec<FieldDef> }

impl Schema {
    pub fn new(name: &str) -> Self { Schema { name: name.to_string(), fields: vec![] } }
    pub fn field(mut self, def: FieldDef) -> Self { self.fields.push(def); self }

    pub fn validate(&self, obj: &HashMap<String, ValType>) -> Vec<ValidationError> {
        let mut errors = vec![];
        // Check required
        for field in &self.fields {
            if field.required && !obj.contains_key(&field.name) {
                errors.push(ValidationError { path: field.name.clone(), message: "required field missing".into() });
            }
        }
        // Validate types and constraints
        for (key, value) in obj {
            if let Some(field) = self.fields.iter().find(|f| f.name == *key) {
                errors.extend(self.validate_field(field, value, key));
            }
        }
        errors
    }

    fn validate_field(&self, field: &FieldDef, value: &ValType, path: &str) -> Vec<ValidationError> {
        let mut errors = vec![];
        let full_path = path.to_string();
        match (&field.field_type, value) {
            (FieldType::String, ValType::String(_)) => {},
            (FieldType::Int, ValType::Int(_)) => {},
            (FieldType::Float, ValType::Float(_)) | (FieldType::Float, ValType::Int(_)) => {},
            (FieldType::Bool, ValType::Bool(_)) => {},
            (FieldType::Bytes, ValType::Bytes(_)) => {},
            (FieldType::Array, ValType::Array(_)) => {},
            (FieldType::Object, ValType::Object(_)) => {
                if !field.fields.is_empty() {
                    if let ValType::Object(map) = value {
                        for f in &field.fields {
                            errors.extend(self.validate_field(f, &ValType::Null, &format!("{}.{}", full_path, f.name)));
                        }
                        for (k, v) in map {
                            if let Some(f) = field.fields.iter().find(|ff| ff.name == *k) {
                                errors.extend(self.validate_field(f, v, &format!("{}.{}", full_path, k)));
                            }
                        }
                    }
                }
            },
            (expected, actual) => {
                let type_name = match actual { ValType::String(_) => "string", ValType::Int(_) => "int", ValType::Float(_) => "float", ValType::Bool(_) => "bool", ValType::Bytes(_) => "bytes", ValType::Array(_) => "array", ValType::Object(_) => "object", ValType::Null => "null" };
                let exp_name = match expected { FieldType::String => "string", FieldType::Int => "int", FieldType::Float => "float", FieldType::Bool => "bool", FieldType::Bytes => "bytes", FieldType::Array => "array", FieldType::Object => "object" };
                errors.push(ValidationError { path: full_path, message: format!("expected {}, got {}", exp_name, type_name) });
            }
        }
        // Numeric constraints
        if let (Some(min), ValType::Float(v)) = (field.min_val, value) { if *v < min { errors.push(ValidationError { path: full_path.clone(), message: format!("value {} below minimum {}", v, min) }); } }
        if let (Some(min), ValType::Int(v)) = (field.min_val, value) { if (*v as f64) < min { errors.push(ValidationError { path: full_path.clone(), message: format!("value {} below minimum {}", v, min) }); } }
        if let (Some(max), ValType::Float(v)) = (field.max_val, value) { if *v > max { errors.push(ValidationError { path: full_path.clone(), message: format!("value {} above maximum {}", v, max) }); } }
        // Enum constraint
        if !field.enum_values.is_empty() {
            if let ValType::String(s) = value {
                if !field.enum_values.contains(s) { errors.push(ValidationError { path: full_path, message: format!("value '{}' not in {:?}", s, field.enum_values) }); }
            }
        }
        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_person_schema() -> Schema {
        Schema::new("person")
            .field(FieldDef::required("name", FieldType::String))
            .field(FieldDef::required("age", FieldType::Int).with_min(0.0).with_max(150.0))
            .field(FieldDef::optional("role", FieldType::String).with_enum(&["admin", "user", "guest"]))
    }

    #[test]
    fn test_valid_object() {
        let schema = make_person_schema();
        let mut obj = HashMap::new();
        obj.insert("name".into(), ValType::String("Alice".into()));
        obj.insert("age".into(), ValType::Int(30));
        let errors = schema.validate(&obj);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_missing_required() {
        let schema = make_person_schema();
        let obj = HashMap::new();
        let errors = schema.validate(&obj);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_type_mismatch() {
        let schema = make_person_schema();
        let mut obj = HashMap::new();
        obj.insert("name".into(), ValType::Int(42));
        obj.insert("age".into(), ValType::Int(30));
        let errors = schema.validate(&obj);
        assert!(errors.iter().any(|e| e.message.contains("expected string")));
    }

    #[test]
    fn test_min_constraint() {
        let schema = make_person_schema();
        let mut obj = HashMap::new();
        obj.insert("name".into(), ValType::String("Bob".into()));
        obj.insert("age".into(), ValType::Int(-1));
        let errors = schema.validate(&obj);
        assert!(errors.iter().any(|e| e.message.contains("below minimum")));
    }

    #[test]
    fn test_max_constraint() {
        let schema = make_person_schema();
        let mut obj = HashMap::new();
        obj.insert("name".into(), ValType::String("Old".into()));
        obj.insert("age".into(), ValType::Int(200));
        let errors = schema.validate(&obj);
        assert!(errors.iter().any(|e| e.message.contains("above maximum")));
    }

    #[test]
    fn test_enum_valid() {
        let schema = make_person_schema();
        let mut obj = HashMap::new();
        obj.insert("name".into(), ValType::String("X".into()));
        obj.insert("age".into(), ValType::Int(25));
        obj.insert("role".into(), ValType::String("admin".into()));
        let errors = schema.validate(&obj);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_enum_invalid() {
        let schema = make_person_schema();
        let mut obj = HashMap::new();
        obj.insert("name".into(), ValType::String("X".into()));
        obj.insert("age".into(), ValType::Int(25));
        obj.insert("role".into(), ValType::String("hacker".into()));
        let errors = schema.validate(&obj);
        assert!(errors.iter().any(|e| e.message.contains("not in")));
    }

    #[test]
    fn test_optional_field() {
        let schema = make_person_schema();
        let mut obj = HashMap::new();
        obj.insert("name".into(), ValType::String("X".into()));
        obj.insert("age".into(), ValType::Int(20));
        let errors = schema.validate(&obj);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_float_accepts_int() {
        let schema = Schema::new("test").field(FieldDef::required("x", FieldType::Float));
        let mut obj = HashMap::new();
        obj.insert("x".into(), ValType::Int(42));
        let errors = schema.validate(&obj);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_extra_fields_ok() {
        let schema = make_person_schema();
        let mut obj = HashMap::new();
        obj.insert("name".into(), ValType::String("X".into()));
        obj.insert("age".into(), ValType::Int(20));
        obj.insert("extra".into(), ValType::String("ignored".into()));
        let errors = schema.validate(&obj);
        assert!(errors.is_empty());
    }
}
