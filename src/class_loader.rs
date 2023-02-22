use crate::class::Class;
use crate::class_file::ClassFile;
use std::fs::File;
use std::io::Read;

pub struct ClassLoader {
    class_path: String,
}

impl ClassLoader {
    pub(crate) fn find_class(&self, name: &str) -> Class {
        let path = format!("{}/{}.class", &self.class_path, name.replace(".", "/"));

        let mut buf = vec![];
        let _ = File::open(path).unwrap().read_to_end(&mut buf);

        self.define_class(buf.as_slice())
    }

    fn define_class(&self, bytes: &[u8]) -> Class {
        let class_file = ClassFile::from(bytes);
        Class::from(class_file)
    }
}

impl ClassLoader {
    pub fn new(class_path: &str) -> ClassLoader {
        ClassLoader {
            class_path: class_path.to_string(),
        }
    }
}
