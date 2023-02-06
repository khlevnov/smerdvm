use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Box<[Constant]>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Box<[u16]>,
    fields_count: u16,
    fields: Box<[FieldInfo]>,
    methods_count: u16,
    methods: Box<[MethodInfo]>,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>,
}

#[derive(Debug)]
pub struct ConstantClass {
    tag: u8,
    name_index: u16,
}

impl From<&[u8]> for ConstantClass {
    fn from(buf: &[u8]) -> Self {
        ConstantClass {
            tag: 7,
            name_index: u16::from_be_bytes(buf[1..3].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct ConstantMethodRefInfo {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
}

impl From<&[u8]> for ConstantMethodRefInfo {
    fn from(buf: &[u8]) -> Self {
        ConstantMethodRefInfo {
            tag: 10,
            class_index: u16::from_be_bytes(buf[1..3].try_into().unwrap()),
            name_and_type_index: u16::from_be_bytes(buf[3..5].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct ConstantNameAndType {
    tag: u8,
    name_index: u16,
    descriptor_index: u16,
}

impl From<&[u8]> for ConstantNameAndType {
    fn from(buf: &[u8]) -> Self {
        ConstantNameAndType {
            tag: 12,
            name_index: u16::from_be_bytes(buf[1..3].try_into().unwrap()),
            descriptor_index: u16::from_be_bytes(buf[3..5].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct ConstantUtf8 {
    tag: u8,
    length: u16,
    bytes: Box<[u8]>,
}

impl From<&[u8]> for ConstantUtf8 {
    fn from(buf: &[u8]) -> Self {
        let length = u16::from_be_bytes(buf[1..3].try_into().unwrap());
        ConstantUtf8 {
            tag: 1,
            length,
            bytes: buf[3..3 + length as usize].to_vec().into_boxed_slice(),
        }
    }
}

#[derive(Debug)]
pub enum Constant {
    Class(ConstantClass),
    MethodRef(ConstantMethodRefInfo),
    NameAndType(ConstantNameAndType),
    Utf8(ConstantUtf8),
}

enum ConstantType {
    Class = 7,
    MethodRef = 10,
    NameAndType = 12,
    Utf8 = 1,
}

#[derive(Debug)]
struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Box<[u8]>,
}

impl AttributeInfo {
    fn size(&self) -> usize {
        6 + self.info.len()
    }
}

impl From<&[u8]> for AttributeInfo {
    fn from(buf: &[u8]) -> Self {
        let attribute_length = u32::from_be_bytes(buf[2..6].try_into().unwrap());
        AttributeInfo {
            attribute_name_index: u16::from_be_bytes(buf[0..2].try_into().unwrap()),
            attribute_length,
            info: buf[6..6 + attribute_length as usize].to_vec().into_boxed_slice(),
        }
    }
}

#[derive(Debug)]
struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>,
}

impl From<&[u8]> for FieldInfo {
    fn from(buf: &[u8]) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>,
}

impl MethodInfo {
    fn size(&self) -> usize {
        8 + self.attributes.iter().map(AttributeInfo::size).sum::<usize>()
    }
}

impl From<&[u8]> for MethodInfo {
    fn from(buf: &[u8]) -> Self {
        let attributes_count = u16::from_be_bytes(buf[6..8].try_into().unwrap());
        let mut offset = 8;
        let mut attributes = vec![];

        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from(&buf[offset..]);
            offset += 6 + attribute_info.attribute_length as usize;
            attributes.push(attribute_info);
        }

        MethodInfo {
            access_flags: u16::from_be_bytes(buf[0..2].try_into().unwrap()),
            name_index: u16::from_be_bytes(buf[2..4].try_into().unwrap()),
            descriptor_index: u16::from_be_bytes(buf[4..6].try_into().unwrap()),
            attributes_count,
            attributes: attributes.into_boxed_slice(),
        }
    }
}

impl From<File> for ClassFile {
    fn from(mut file: File) -> Self {
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();

        let magic = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        assert_eq!(magic, 0xcafebabe); // TODO: add message

        let constant_pool_count = u16::from_be_bytes(buf[8..10].try_into().unwrap());

        let mut offset = 10;
        let mut constant_pool = vec![];

        for _ in 1..constant_pool_count {
            let tag = u8::from_be_bytes(buf[offset..offset + 1].try_into().unwrap());
            let constant = match tag {
                1 => {
                    let constant = ConstantUtf8::from(&buf[offset..]);
                    offset += 3 + constant.bytes.len();
                    Constant::Utf8(constant)
                },
                7 => {
                    let constant = ConstantClass::from(&buf[offset..]);
                    offset += 3;
                    Constant::Class(constant)
                },
                10 => {
                    let constant = ConstantMethodRefInfo::from(&buf[offset..]);
                    offset += 5;
                    Constant::MethodRef(constant)
                },
                12 => {
                    let constant = ConstantNameAndType::from(&buf[offset..]);
                    offset += 5;
                    Constant::NameAndType(constant)
                },
                _ => unreachable!()
            };
            constant_pool.push(constant);
        }

        let access_flags = u16::from_be_bytes(buf[offset..offset + 2].try_into().unwrap());
        let this_class = u16::from_be_bytes(buf[offset + 2..offset + 4].try_into().unwrap());
        let super_class = u16::from_be_bytes(buf[offset + 4..offset + 6].try_into().unwrap());

        let interfaces_count = u16::from_be_bytes(buf[offset + 6..offset + 8].try_into().unwrap());
        offset += 8;

        for _ in 0..interfaces_count {
            offset += 0;
        }

        let fields_count = u16::from_be_bytes(buf[offset..offset + 2].try_into().unwrap());
        offset += 2;

        for _ in 0..fields_count {
            offset += 0;
        }

        let methods_count = u16::from_be_bytes(buf[offset..offset + 2].try_into().unwrap());
        offset += 2;

        let mut methods = vec![];
        for _ in 0..methods_count {
            let method_info = MethodInfo::from(&buf[offset..]);
            offset += method_info.size();
            methods.push(method_info);
        }

        let attributes_count = u16::from_be_bytes(buf[offset..offset + 2].try_into().unwrap());
        offset += 2;

        let mut attributes = vec![];
        for _ in 0..methods_count {
            let attribute_info = AttributeInfo::from(&buf[offset..]);
            offset += attribute_info.size();
            attributes.push(attribute_info);
        }

        let class_file = ClassFile {
            magic,
            minor_version: u16::from_be_bytes(buf[4..6].try_into().unwrap()),
            major_version: u16::from_be_bytes(buf[6..8].try_into().unwrap()),
            constant_pool_count,
            constant_pool: constant_pool.into_boxed_slice(),
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces: Box::new([]),
            fields_count,
            fields: Box::new([]),
            methods_count,
            methods: methods.into_boxed_slice(),
            attributes_count,
            attributes: attributes.into_boxed_slice(),
        };

        class_file
    }
}
