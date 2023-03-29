use super::j_value::JValue;

/// JObject is a trait that defines the methods that all Java objects must implement
pub trait JObject {
    fn identify(&self) -> String;
    fn get(&self, key: &str) -> Option<JValue>;
    fn get_mut(&mut self, key: &str) -> Option<&mut JValue>;
    fn set(&mut self, key: &str, value: JValue) -> Option<()>;

    
}

