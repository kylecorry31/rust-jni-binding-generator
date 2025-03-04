use converter::Converter;
use float_converter::FloatConverter;
use int_converter::IntConverter;

pub mod converter;
pub mod float_converter;
pub mod int_converter;

pub fn get_converter(rust_type: &str) -> Box<dyn Converter> {
    let int = IntConverter {};
    let float = FloatConverter {};

    if int.can_process_type(rust_type) {
        return Box::new(int);
    }

    if float.can_process_type(rust_type) {
        return Box::new(float);
    }

    panic!("No processor found");
}
