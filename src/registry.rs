use crate::class::Class;
use crate::r#type::Type;
use std::collections::HashMap;

pub type NativeMethod = fn(&[Type]) -> Type;

#[derive(Default)]
pub struct Registry {
    pub(crate) classes: HashMap<String, Class>,
    pub(crate) native_methods: HashMap<String, NativeMethod>,
}

impl Registry {
    pub fn add_class(&mut self, class: Class) -> &mut Self {
        self.classes.insert(class.this_class(), class);
        self
    }

    pub fn add_native(&mut self, name: &str, native_method: NativeMethod) -> &mut Self {
        self.native_methods.insert(name.into(), native_method);
        self
    }

    pub fn class(&self, name: &str) -> &Class {
        &self.classes.get(name).expect(&*format!(
            "java.lang.ClassNotFoundException: {}",
            name.replace("/", ".")
        ))
    }

    pub fn native_method(&self, name: &str) -> &NativeMethod {
        &self.native_methods[name]
    }
}
