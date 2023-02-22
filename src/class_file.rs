use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::str::from_utf8;

#[derive(Debug)]
pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    pub(crate) constant_pool: Vec<ConstantInfo>,
    pub(crate) access_flags: u16,
    pub(crate) this_class: u16,
    pub(crate) super_class: u16,
    pub(crate) interfaces: Box<[u16]>,
    pub(crate) fields: Vec<FieldInfo>,
    pub(crate) methods: Vec<MethodInfo>,
    pub(crate) attributes: Vec<AttributeInfo>,
}

pub trait ClassReader: Read {
    fn read_u1(&mut self) -> u8;
    fn read_u2(&mut self) -> u16;
    fn read_u4(&mut self) -> u32;
    fn cp_info(&mut self) -> ConstantInfo;
}

impl<T> ClassReader for BufReader<T>
where
    T: Read,
{
    fn read_u1(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf).unwrap();
        u8::from_be_bytes(buf)
    }

    fn read_u2(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf).unwrap();
        u16::from_be_bytes(buf)
    }

    fn read_u4(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf).unwrap();
        u32::from_be_bytes(buf)
    }

    fn cp_info(&mut self) -> ConstantInfo {
        match self.read_u1() {
            7 => ConstantInfo::Class(self.into()),
            9 => ConstantInfo::FieldRef(self.into()),
            10 => ConstantInfo::MethodRef(self.into()),
            8 => ConstantInfo::String(self.into()),
            3 => ConstantInfo::Integer(self.into()),
            4 => ConstantInfo::Float(self.into()),
            5 => ConstantInfo::Long(self.into()),
            6 => ConstantInfo::Double(self.into()),
            12 => ConstantInfo::NameAndType(self.into()),
            1 => ConstantInfo::Utf8(self.into()),
            tag => {
                println!("tag {}", tag);
                unreachable!()
            }
        }
    }
}

#[derive(Debug)]
pub struct ConstantClassInfo {
    pub(crate) name_index: u16,
}

impl<T> From<&mut T> for ConstantClassInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        ConstantClassInfo {
            name_index: reader.read_u2(),
        }
    }
}

#[derive(Debug)]
pub struct ConstantFieldRefInfo {
    pub(crate) class_index: u16,
    pub(crate) name_and_type_index: u16,
}

impl<T> From<&mut T> for ConstantFieldRefInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        ConstantFieldRefInfo {
            class_index: reader.read_u2(),
            name_and_type_index: reader.read_u2(),
        }
    }
}

#[derive(Debug)]
pub struct ConstantMethodRefInfo {
    pub(crate) class_index: u16,
    pub(crate) name_and_type_index: u16,
}

impl<T> From<&mut T> for ConstantMethodRefInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        ConstantMethodRefInfo {
            class_index: reader.read_u2(),
            name_and_type_index: reader.read_u2(),
        }
    }
}

#[derive(Debug)]
pub struct ConstantNameAndTypeInfo {
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
}

impl<T> From<&mut T> for ConstantNameAndTypeInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        ConstantNameAndTypeInfo {
            name_index: reader.read_u2(),
            descriptor_index: reader.read_u2(),
        }
    }
}

#[derive(Debug)]
pub struct ConstantStringInfo {
    pub(crate) string_index: u16,
}

impl<T> From<&mut T> for ConstantStringInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        ConstantStringInfo {
            string_index: reader.read_u2(),
        }
    }
}

#[derive(Debug)]
pub struct ConstantIntegerInfo {
    pub(crate) value: i32,
}

impl<T> From<&mut T> for ConstantIntegerInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).unwrap();

        ConstantIntegerInfo {
            value: i32::from_be_bytes(buf),
        }
    }
}

#[derive(Debug)]
pub struct ConstantFloatInfo {
    pub(crate) value: f32,
}

impl<T> From<&mut T> for ConstantFloatInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).unwrap();

        ConstantFloatInfo {
            value: f32::from_be_bytes(buf),
        }
    }
}

#[derive(Debug)]
pub struct ConstantLongInfo {
    pub(crate) value: i64,
}

impl<T> From<&mut T> for ConstantLongInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf).unwrap();

        ConstantLongInfo {
            value: i64::from_be_bytes(buf),
        }
    }
}

#[derive(Debug)]
pub struct ConstantDoubleInfo {
    pub(crate) value: f64,
}

impl<T> From<&mut T> for ConstantDoubleInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf).unwrap();

        ConstantDoubleInfo {
            value: f64::from_be_bytes(buf),
        }
    }
}

#[derive(Debug)]
pub struct ConstantUtf8Info {
    pub(crate) value: String,
}

impl<T> From<&mut T> for ConstantUtf8Info
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        let length = reader.read_u2();

        let mut buf = vec![0u8; length as usize];
        reader.read_exact(&mut buf).unwrap();

        ConstantUtf8Info {
            value: from_utf8(&buf).unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ConstantInfo {
    Class(ConstantClassInfo),
    FieldRef(ConstantFieldRefInfo),
    MethodRef(ConstantMethodRefInfo),
    String(ConstantStringInfo),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    NameAndType(ConstantNameAndTypeInfo),
    Utf8(ConstantUtf8Info),
    Unusable,
}

#[derive(Debug)]
pub(crate) struct AttributeInfo {
    pub(crate) attribute_name_index: u16,
    pub(crate) info: Box<[u8]>,
}

impl<T> From<&mut T> for AttributeInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        let attribute_name_index = reader.read_u2();
        let attribute_length = reader.read_u4();

        let mut buf = vec![0u8; attribute_length as usize];
        reader.read_exact(&mut buf).unwrap();

        AttributeInfo {
            attribute_name_index,
            info: buf.into_boxed_slice(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>,
}

impl<T> From<&mut T> for FieldInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        let access_flags = reader.read_u2();
        let name_index = reader.read_u2();
        let descriptor_index = reader.read_u2();
        let attributes_count = reader.read_u2();

        FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes: (0..attributes_count)
                .into_iter()
                .map(|_| AttributeInfo::from(&mut *reader))
                .collect::<Box<[AttributeInfo]>>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MethodInfo {
    pub(crate) access_flags: u16,
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
    pub(crate) attributes: Vec<AttributeInfo>,
}

impl<T> From<&mut T> for MethodInfo
where
    T: ClassReader,
{
    fn from(reader: &mut T) -> Self {
        let access_flags = reader.read_u2();
        let name_index = reader.read_u2();
        let descriptor_index = reader.read_u2();
        let attributes_count = reader.read_u2();

        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes: (0..attributes_count)
                .into_iter()
                .map(|_| AttributeInfo::from(&mut *reader))
                .collect(),
        }
    }
}

impl<T> From<T> for ClassFile
where
    T: ClassReader,
{
    fn from(mut reader: T) -> Self {
        let magic = reader.read_u4();
        assert_eq!(
            magic, 0xcafebabe,
            "Incompatible magic value {:#x} in class file Foo",
            magic
        );

        let minor_version = reader.read_u2();
        let major_version = reader.read_u2();
        let constant_pool_count = reader.read_u2();

        let mut constant_pool = vec![];
        let mut index = 1;
        while index < constant_pool_count {
            let cp_info = reader.cp_info();
            if matches!(cp_info, ConstantInfo::Long(_) | ConstantInfo::Double(_)) {
                constant_pool.push(cp_info);
                constant_pool.push(ConstantInfo::Unusable);
                index += 2;
            } else {
                constant_pool.push(cp_info);
                index += 1;
            }
        }

        let access_flags = reader.read_u2();
        let this_class = reader.read_u2();
        let super_class = reader.read_u2();

        let interfaces_count = reader.read_u2();
        let interfaces = (0..interfaces_count)
            .into_iter()
            .map(|_| reader.read_u2())
            .collect();

        let fields_count = reader.read_u2();
        let fields = (0..fields_count)
            .into_iter()
            .map(|_| FieldInfo::from(&mut reader))
            .collect();

        let methods_count = reader.read_u2();
        let methods = (0..methods_count)
            .into_iter()
            .map(|_| MethodInfo::from(&mut reader))
            .collect();

        let attributes_count = reader.read_u2();
        let attributes = (0..attributes_count)
            .into_iter()
            .map(|_| AttributeInfo::from(&mut reader))
            .collect();

        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }
}

impl From<File> for ClassFile {
    fn from(f: File) -> Self {
        BufReader::new(f).into()
    }
}

impl From<&[u8]> for ClassFile {
    fn from(bytes: &[u8]) -> Self {
        BufReader::new(bytes).into()
    }
}
