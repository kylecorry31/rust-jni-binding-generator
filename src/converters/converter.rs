pub trait Converter {
    fn can_process_type(&self, rust_type: &str) -> bool;
    fn get_jni_type(&self) -> String;
    fn get_kotlin_type(&self) -> String;
    fn convert_from_jni_to_rust(&self, jni_variable_name: &str) -> String {
        jni_variable_name.to_string()
    }
    fn convert_from_rust_to_jni(&self, rust_variable_name: &str) -> String {
        rust_variable_name.to_string()
    }
}
