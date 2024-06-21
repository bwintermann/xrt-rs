use crate::buffer::*;
pub trait HardwareDatatype {}

impl HardwareDatatype for u32 {}
impl HardwareDatatype for i32 {}
impl HardwareDatatype for u64 {}
impl HardwareDatatype for i64 {}
impl HardwareDatatype for f32 {}
impl HardwareDatatype for f64 {}


/// Actual argument data
pub enum Argument<'a, T: HardwareDatatype> {
    Scalar(T),
    Buffer(&'a [T]),
}

/// Type of argument
pub enum ArgumentType {
    Scalar,
    Buffer(Box<XRTBuffer<dyn HardwareDatatype>>)
}

/// This type is used to save what arguments at which position have what type
pub type ArgumentMapping = Vec<ArgumentType>;