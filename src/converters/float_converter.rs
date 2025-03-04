use super::converter::Converter;

pub struct FloatConverter {}

impl Converter for FloatConverter {
    fn can_process_type(&self, rust_type: &str) -> bool {
        rust_type == "f32"
    }

    fn get_jni_type(&self) -> String {
        "jfloat".to_string()
    }

    fn get_kotlin_type(&self) -> String {
        "Float".to_string()
    }
}
