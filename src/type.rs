#[derive(Clone, Copy, Debug)]
pub enum Type {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Null,
    Void,
}

impl Type {
    pub(crate) fn int(self) -> i32 {
        match self {
            Type::Int(x) => x,
            _ => panic!(),
        }
    }

    pub(crate) fn float(self) -> f32 {
        match self {
            Type::Float(x) => x,
            _ => panic!(),
        }
    }

    pub(crate) fn long(self) -> i64 {
        match self {
            Type::Long(x) => x,
            _ => panic!(),
        }
    }

    pub(crate) fn double(self) -> f64 {
        match self {
            Type::Double(x) => x,
            _ => panic!(),
        }
    }
}
