use super::converter::Converter;

pub struct IntConverter {}

impl Converter for IntConverter {
    fn can_process_type(&self, rust_type: &str) -> bool {
        rust_type == "i32"
    }

    fn get_jni_type(&self) -> String {
        "jint".to_string()
    }

    fn get_kotlin_type(&self) -> String {
        "Int".to_string()
    }
}
