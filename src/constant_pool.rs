use crate::class_file::{
    ConstantClassInfo, ConstantDoubleInfo, ConstantFieldRefInfo, ConstantFloatInfo, ConstantInfo,
    ConstantIntegerInfo, ConstantLongInfo, ConstantMethodRefInfo, ConstantNameAndTypeInfo,
    ConstantStringInfo,
};
use crate::r#type::Type;

#[derive(Debug)]
pub enum Constant {
    Class(ConstantClassInfo),
    FieldRef(ConstantFieldRefInfo),
    MethodRef(ConstantMethodRefInfo),
    String(ConstantStringInfo),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    NameAndType(ConstantNameAndTypeInfo),
    Utf8(String),
    Unusable,
}

impl From<ConstantInfo> for Constant {
    fn from(info: ConstantInfo) -> Self {
        match info {
            ConstantInfo::Class(info) => Constant::Class(info),
            ConstantInfo::FieldRef(info) => Constant::FieldRef(info),
            ConstantInfo::MethodRef(info) => Constant::MethodRef(info),
            ConstantInfo::String(info) => Constant::String(info),
            ConstantInfo::Integer(info) => Constant::Integer(info),
            ConstantInfo::Float(info) => Constant::Float(info),
            ConstantInfo::Long(info) => Constant::Long(info),
            ConstantInfo::Double(info) => Constant::Double(info),
            ConstantInfo::NameAndType(info) => Constant::NameAndType(info),
            ConstantInfo::Utf8(info) => Constant::Utf8(info.value),
            ConstantInfo::Unusable => Constant::Unusable,
        }
    }
}

#[derive(Debug)]
pub struct ConstantPool {
    constant_pool: Vec<Constant>,
}

impl ConstantPool {
    pub(crate) fn classes(&self) -> Vec<String> {
        self.constant_pool
            .iter()
            .filter_map(|constant| match constant {
                Constant::Class(class_info) => Some(self.utf8(class_info.name_index)),
                _ => None,
            })
            .collect::<Vec<String>>()
    }

    pub(crate) fn class(&self, index: u16) -> String {
        match &self.constant_pool[index as usize - 1] {
            Constant::Class(info) => self.utf8(info.name_index),
            _ => unreachable!(),
        }
    }

    pub(crate) fn method_ref(&self, index: u16) -> (String, String) {
        match &self.constant_pool[index as usize - 1] {
            Constant::MethodRef(info) => {
                let class = self.class(info.class_index);
                let name_and_type = self.name_and_type(info.name_and_type_index);
                (class, name_and_type)
            }
            _ => unreachable!(),
        }
    }

    fn name_and_type(&self, index: u16) -> String {
        match &self.constant_pool[index as usize - 1] {
            Constant::NameAndType(info) => {
                let name = self.utf8(info.name_index);
                let descriptor = self.utf8(info.descriptor_index);
                format!("{}:{}", name, descriptor)
            }
            _ => unreachable!(),
        }
    }

    pub fn utf8(&self, index: u16) -> String {
        match &self.constant_pool[index as usize - 1] {
            Constant::Utf8(value) => value.to_owned(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn get(&self, index: usize) -> Type {
        match &self.constant_pool[index as usize - 1] {
            Constant::Integer(x) => Type::Int(x.value),
            Constant::Float(x) => Type::Float(x.value),
            Constant::Long(x) => Type::Long(x.value),
            Constant::Double(x) => Type::Double(x.value),
            _ => unreachable!(),
        }
    }
}

impl From<Vec<ConstantInfo>> for ConstantPool {
    fn from(constant_pool: Vec<ConstantInfo>) -> Self {
        ConstantPool {
            constant_pool: constant_pool.into_iter().map(Constant::from).collect(),
        }
    }
}
