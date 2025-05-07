use dora_node_api::Metadata;
use dora_node_api::Parameter;

// 定义一个扩展trait
pub trait MetadataExt {
    fn get(&self, key: &str) -> Option<&Parameter>;
    fn get_string(&self, key: &str, default: &str) -> String;
    fn get_bool(&self, key: &str, default: bool) -> bool;
    fn get_int(&self, key: &str, default: i64) -> i64;
    fn get_float(&self, key: &str, default: f64) -> f64;
    fn get_list_int(&self, key: &str, default: Vec<i64>) -> Vec<i64>;
    fn get_list_float(&self, key: &str, default: Vec<f64>) -> Vec<f64>;
    fn get_list_string(&self, key: &str, default: Vec<String>) -> Vec<String>;
}

// 为Metadata实现这个trait
impl MetadataExt for Metadata {
    fn get(&self, key: &str) -> Option<&Parameter> {
        self.parameters.get(key)
    }

    fn get_string(&self, key: &str, default: &str) -> String {
        if let Some(Parameter::String(value)) = self.parameters.get(key) {
            value.clone()
        } else {
            default.to_string()
        }
    }

    fn get_bool(&self, key: &str, default: bool) -> bool {
        if let Some(Parameter::Bool(value)) = self.parameters.get(key) {
            *value
        } else {
            default
        }
    }

    fn get_int(&self, key: &str, default: i64) -> i64 {
        if let Some(Parameter::Integer(value)) = self.parameters.get(key) {
            *value
        } else {
            default
        }
    }

    fn get_float(&self, key: &str, default: f64) -> f64 {
        match self.parameters.get(key) {
            Some(Parameter::Float(value)) => *value,
            Some(Parameter::Integer(value)) => *value as f64,
            _ => default,
        }
    }

    fn get_list_int(&self, key: &str, default: Vec<i64>) -> Vec<i64> {
        if let Some(Parameter::ListInt(value)) = self.parameters.get(key) {
            value.clone()
        } else {
            default
        }
    }

    fn get_list_float(&self, key: &str, default: Vec<f64>) -> Vec<f64> {
        if let Some(Parameter::ListFloat(value)) = self.parameters.get(key) {
            value.clone()
        } else {
            default
        }
    }

    fn get_list_string(&self, key: &str, default: Vec<String>) -> Vec<String> {
        if let Some(Parameter::ListString(value)) = self.parameters.get(key) {
            value.clone()
        } else {
            default
        }
    }
}
