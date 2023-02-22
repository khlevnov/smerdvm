use std::io::BufReader;
use crate::class_file::{AttributeInfo, ClassReader, MethodInfo};
use crate::constant_pool::{Constant, ConstantPool};

const ACC_NATIVE: u16 = 0x0100;
const ACC_ABSTRACT: u16 = 0x0400;
const CODE_ATTRIBUTE_NAME: &str = "Code";

#[derive(Debug)]
pub struct Method {
    pub(crate) access_flags: u16,
    pub(crate) name: String,
    pub(crate) descriptor: String,
    pub(crate) max_stack: usize,
    pub(crate) max_locals: usize,
    pub(crate) code: Vec<u8>,
}

#[derive(Debug, Default)]
struct CodeAttribute {
    max_stack: usize,
    max_locals: usize,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<AttributeInfo>,
}

impl<T> From<T> for CodeAttribute
    where
        T: ClassReader,
{
    fn from(mut reader: T) -> Self {
        let max_stack = reader.read_u2() as usize;
        let max_locals = reader.read_u2() as usize;
        let code_length = reader.read_u4();

        let mut code = vec![0u8; code_length as usize];
        reader.read_exact(&mut code).unwrap();

        let exception_table_length = reader.read_u2();
        let exception_table = (0..exception_table_length)
            .into_iter()
            .map(|_| ExceptionTableEntry::from(&mut reader))
            .collect::<Vec<ExceptionTableEntry>>();

        let attributes_count = reader.read_u2();
        let attributes = (0..attributes_count)
            .into_iter()
            .map(|_| AttributeInfo::from(&mut reader))
            .collect::<Vec<AttributeInfo>>();

        CodeAttribute {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        }
    }
}

#[derive(Debug)]
struct ExceptionTableEntry {
    start_pc: usize,
    end_pc: usize,
    handler_pc: usize,
    catch_type: u16,
}

impl<T> From<&mut T> for ExceptionTableEntry
    where
        T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        ExceptionTableEntry {
            start_pc: reader.read_u2() as usize,
            end_pc: reader.read_u2() as usize,
            handler_pc: reader.read_u2() as usize,
            catch_type: reader.read_u2(),
        }
    }
}

impl Method {
    pub(crate) fn is_native(&self) -> bool {
        self.access_flags & ACC_NATIVE != 0
    }

    pub(crate) fn from(mut method_info: MethodInfo, constant_pool: &ConstantPool) -> Method {
        let CodeAttribute {
            max_stack,
            max_locals,
            code,
            ..
        } = code_attribute(method_info.access_flags, &mut method_info.attributes, constant_pool);

        Method {
            access_flags: method_info.access_flags,
            name: constant_pool.utf8(method_info.name_index),
            descriptor: constant_pool.utf8(method_info.descriptor_index),
            max_stack,
            max_locals,
            code,
        }
    }
}

fn code_attribute(
    access_flags: u16,
    attributes: &mut Vec<AttributeInfo>,
    constant_pool: &ConstantPool) -> CodeAttribute
{
    if access_flags & (ACC_NATIVE | ACC_ABSTRACT) != 0 {
        return CodeAttribute::default()
    }

    let code_attribute_index = attributes.iter()
        .map(|info| info.attribute_name_index)
        .map(|index| constant_pool.utf8(index))
        .enumerate()
        .find(|(_, name)| name == CODE_ATTRIBUTE_NAME)
        .map(|(index, _)| index)
        .unwrap();

    let attribute_info = attributes.swap_remove(code_attribute_index);
    BufReader::new(&attribute_info.info[..]).into()
}
