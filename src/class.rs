use crate::class_file::ClassFile;
use crate::constant_pool::ConstantPool;
use crate::method::Method;
use crate::thread::Thread;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Class {
    access_flags: u16,
    pub(crate) constant_pool: ConstantPool,
    pub(crate) methods: HashMap<String, Method>,
    this_class: u16,
    super_class: u16,
}

impl Class {
    pub fn method(&self, name_and_type: &str) -> Option<&Method> {
        self.methods.get(name_and_type)
    }

    pub fn super_class(&self) -> String {
        self.constant_pool.class(self.super_class)
    }

    pub fn this_class(&self) -> String {
        self.constant_pool.class(self.this_class)
    }
}

impl From<ClassFile> for Class {
    fn from(class_file: ClassFile) -> Self {
        let ClassFile {
            access_flags,
            constant_pool,
            this_class,
            super_class,
            methods,
            ..
        } = class_file;

        let constant_pool = ConstantPool::from(constant_pool);
        Class {
            access_flags,
            methods: methods
                .into_iter()
                .map(|info| Method::from(info, &constant_pool))
                .map(|method| (format!("{}:{}", method.name, method.descriptor), method))
                .collect(),
            constant_pool,
            this_class,
            super_class,
        }
    }
}
