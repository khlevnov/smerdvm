use crate::constant_pool::ConstantPool;
use crate::method::Method;
use crate::r#type::Type;
use crate::r#type::Type::{Double, Float, Int, Long, Null};

#[derive(Debug)]
pub struct Frame<'a> {
    pub(crate) code: &'a Vec<u8>,
    pub(crate) constant_pool: &'a ConstantPool,
    pub(crate) locals: Vec<Type>,
    pub(crate) operand_stack: Vec<Type>,
    pub(crate) return_address: usize,
}

impl Frame<'_> {
    pub(crate) fn from<'a>(method: &'a Method, constant_pool: &'a ConstantPool) -> Frame<'a> {
        Frame {
            code: &method.code,
            constant_pool,
            locals: vec![Null; method.max_locals],
            operand_stack: Vec::with_capacity(method.max_stack),
            return_address: 0,
        }
    }
}
