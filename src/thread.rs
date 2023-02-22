use crate::constant_pool::ConstantPool;
use crate::frame::Frame;
use crate::r#type::Type;
use crate::r#type::Type::{Double, Float, Int, Long, Void};
use crate::registry::Registry;
use regex::Regex;

const NOP: u8 = 0x00;
const ICONST_M1: u8 = 0x02;
const ICONST_0: u8 = 0x03;
const ICONST_1: u8 = 0x04;
const ICONST_2: u8 = 0x05;
const ICONST_3: u8 = 0x06;
const ICONST_4: u8 = 0x07;
const ICONST_5: u8 = 0x08;
const LCONST_0: u8 = 0x09;
const LCONST_1: u8 = 0x0a;
const FCONST_0: u8 = 0x0b;
const FCONST_1: u8 = 0x0c;
const FCONST_2: u8 = 0x0d;
const DCONST_0: u8 = 0x0e;
const DCONST_1: u8 = 0x0f;
const BIPUSH: u8 = 0x10;
const SIPUSH: u8 = 0x11;
const LDC: u8 = 0x12;
const LDC_W: u8 = 0x13;
const LDC2_W: u8 = 0x14;
const ILOAD: u8 = 0x15;
const LLOAD: u8 = 0x16;
const FLOAD: u8 = 0x17;
const DLOAD: u8 = 0x18;
const ILOAD_0: u8 = 0x1a;
const ILOAD_1: u8 = 0x1b;
const ILOAD_2: u8 = 0x1c;
const ILOAD_3: u8 = 0x1d;
const LLOAD_0: u8 = 0x1e;
const LLOAD_1: u8 = 0x1f;
const LLOAD_2: u8 = 0x20;
const LLOAD_3: u8 = 0x21;
const FLOAD_0: u8 = 0x22;
const FLOAD_1: u8 = 0x23;
const FLOAD_2: u8 = 0x24;
const FLOAD_3: u8 = 0x25;
const DLOAD_0: u8 = 0x26;
const DLOAD_1: u8 = 0x27;
const DLOAD_2: u8 = 0x28;
const DLOAD_3: u8 = 0x29;
const ISTORE: u8 = 0x36;
const LSTORE: u8 = 0x37;
const FSTORE: u8 = 0x38;
const DSTORE: u8 = 0x39;
const ISTORE_0: u8 = 0x3b;
const ISTORE_1: u8 = 0x3c;
const ISTORE_2: u8 = 0x3d;
const ISTORE_3: u8 = 0x3e;
const LSTORE_0: u8 = 0x3f;
const LSTORE_1: u8 = 0x40;
const LSTORE_2: u8 = 0x41;
const LSTORE_3: u8 = 0x42;
const FSTORE_0: u8 = 0x43;
const FSTORE_1: u8 = 0x44;
const FSTORE_2: u8 = 0x45;
const FSTORE_3: u8 = 0x46;
const DSTORE_0: u8 = 0x47;
const DSTORE_1: u8 = 0x48;
const DSTORE_2: u8 = 0x49;
const DSTORE_3: u8 = 0x4a;
const IADD: u8 = 0x60;
const LADD: u8 = 0x61;
const FADD: u8 = 0x62;
const DADD: u8 = 0x63;
const ISUB: u8 = 0x64;
const LSUB: u8 = 0x65;
const FSUB: u8 = 0x66;
const DSUB: u8 = 0x67;
const IMUL: u8 = 0x68;
const LMUL: u8 = 0x69;
const FMUL: u8 = 0x6a;
const DMUL: u8 = 0x6b;
const IDIV: u8 = 0x6c;
const LDIV: u8 = 0x6d;
const FDIV: u8 = 0x6e;
const DDIV: u8 = 0x6f;
const IREM: u8 = 0x70;
const LREM: u8 = 0x71;
const FREM: u8 = 0x72;
const DREM: u8 = 0x73;
const IAND: u8 = 0x7e;
const LAND: u8 = 0x7f;
const IOR: u8 = 0x80;
const LOR: u8 = 0x81;
const I2F: u8 = 0x86;
const IFEQ: u8 = 0x99;
const IFNE: u8 = 0x9a;
const IFLT: u8 = 0x9b;
const IFGE: u8 = 0x9c;
const IFGT: u8 = 0x9d;
const IFLE: u8 = 0x9e;
const IF_ICMPEQ: u8 = 0x9f;
const IF_ICMPNE: u8 = 0xa0;
const IF_ICMPLT: u8 = 0xa1;
const IF_ICMPGE: u8 = 0xa2;
const IF_ICMPGT: u8 = 0xa3;
const IF_ICMPLE: u8 = 0xa4;
const GOTO: u8 = 0xa7;
const IRETURN: u8 = 0xac;
const LRETURN: u8 = 0xad;
const FRETURN: u8 = 0xae;
const DRETURN: u8 = 0xaf;
const RETURN: u8 = 0xb1;
const INVOKESTATIC: u8 = 0xb8;

pub struct Thread<'a> {
    frame_stack: Vec<Frame<'a>>,
    pc: usize,
    registry: &'a Registry,
}

impl Thread<'_> {
    pub(crate) fn run(mut self) -> Type {
        // println!("code {:?}", self.code());
        loop {
            let instruction = self.code()[self.pc];
            self.pc += 1;
            // println!("op {} stack {:?}", instruction, self.operand_stack());
            match instruction {
                NOP => (),
                ICONST_M1 => self.push(Int(-1)),
                ICONST_0 => self.push(Int(0)),
                ICONST_1 => self.push(Int(1)),
                ICONST_2 => self.push(Int(2)),
                ICONST_3 => self.push(Int(3)),
                ICONST_4 => self.push(Int(4)),
                ICONST_5 => self.push(Int(5)),
                LCONST_0 => self.push(Long(0)),
                LCONST_1 => self.push(Long(1)),
                FCONST_0 => self.push(Float(0.0)),
                FCONST_1 => self.push(Float(1.0)),
                FCONST_2 => self.push(Float(2.0)),
                DCONST_0 => self.push(Double(0.0)),
                DCONST_1 => self.push(Double(1.0)),
                BIPUSH => {
                    let bytes = self.read_u8().to_be_bytes();
                    self.push(Int(i8::from_be_bytes(bytes) as i32))
                }
                SIPUSH => {
                    let bytes = self.read_u16().to_be_bytes();
                    self.push(Int(i16::from_be_bytes(bytes) as i32))
                }
                LDC => {
                    let index = self.read_u8();
                    let constant = self.constant_pool().get(index as usize);
                    self.push(constant)
                }
                LDC_W | LDC2_W => {
                    let index = self.read_u16();
                    let constant = self.constant_pool().get(index as usize);
                    self.push(constant)
                }
                ILOAD | LLOAD | FLOAD | DLOAD => {
                    let index = self.read_u8() as usize;
                    let variable = self.locals()[index];
                    self.push(variable)
                }
                ILOAD_0 | LLOAD_0 | FLOAD_0 | DLOAD_0 => self.load_n(0),
                ILOAD_1 | LLOAD_1 | FLOAD_1 | DLOAD_1 => self.load_n(1),
                ILOAD_2 | LLOAD_2 | FLOAD_2 | DLOAD_2 => self.load_n(2),
                ILOAD_3 | LLOAD_3 | FLOAD_3 | DLOAD_3 => self.load_n(3),
                ISTORE | LSTORE | FSTORE | DSTORE => {
                    let index = self.read_u8() as usize;
                    self.locals()[index] = self.pop()
                }
                ISTORE_0 | LSTORE_0 | FSTORE_0 | DSTORE_0 => self.locals()[0] = self.pop(),
                ISTORE_1 | LSTORE_1 | FSTORE_1 | DSTORE_1 => self.locals()[1] = self.pop(),
                ISTORE_2 | LSTORE_2 | FSTORE_2 | DSTORE_2 => self.locals()[2] = self.pop(),
                ISTORE_3 | LSTORE_3 | FSTORE_3 | DSTORE_3 => self.locals()[3] = self.pop(),
                IADD => self.fn2(|a, b| Int(a.int().wrapping_add(b.int()))),
                LADD => self.fn2(|a, b| Long(a.long().wrapping_add(b.long()))),
                FADD => self.fn2(|a, b| Float(a.float() + b.float())),
                DADD => self.fn2(|a, b| Double(a.double() + b.double())),
                ISUB => self.fn2(|a, b| Int(a.int().wrapping_sub(b.int()))),
                LSUB => self.fn2(|a, b| Long(a.long().wrapping_sub(b.long()))),
                FSUB => self.fn2(|a, b| Float(a.float() - b.float())),
                DSUB => self.fn2(|a, b| Double(a.double() - b.double())),
                IMUL => self.fn2(|a, b| Int(a.int().wrapping_mul(b.int()))),
                LMUL => self.fn2(|a, b| Long(a.long().wrapping_mul(b.long()))),
                FMUL => self.fn2(|a, b| Float(a.float() * b.float())),
                DMUL => self.fn2(|a, b| Double(a.double() * b.double())),
                IDIV => self.fn2(|a, b| Int(a.int().wrapping_div(b.int()))),
                LDIV => self.fn2(|a, b| Long(a.long().wrapping_div(b.long()))),
                FDIV => self.fn2(|a, b| Float(a.float() * b.float())),
                DDIV => self.fn2(|a, b| Double(a.double() * b.double())),
                IREM => self.fn2(|a, b| Int(a.int().wrapping_rem(b.int()))),
                LREM => self.fn2(|a, b| Long(a.long().wrapping_rem(b.long()))),
                FREM => self.fn2(|a, b| Float(a.float() % b.float())),
                DREM => self.fn2(|a, b| Double(a.double() % b.double())),
                IAND => self.fn2(|a, b| Int(a.int() & b.int())),
                LAND => self.fn2(|a, b| Long(a.long() & b.long())),
                IOR => self.fn2(|a, b| Int(a.int() | b.int())),
                LOR => self.fn2(|a, b| Long(a.long() | b.long())),
                I2F => self.cast(|x| Float(x.int() as f32)),
                IFEQ => self.jmp_cmp1(|x| x.int() == 0),
                IFNE => self.jmp_cmp1(|x| x.int() != 0),
                IFLT => self.jmp_cmp1(|x| x.int() < 0),
                IFGE => self.jmp_cmp1(|x| x.int() >= 0),
                IFGT => self.jmp_cmp1(|x| x.int() > 0),
                IFLE => self.jmp_cmp1(|x| x.int() <= 0),
                IF_ICMPEQ => self.jmp_cmp2(|a, b| a.int() == b.int()),
                IF_ICMPNE => self.jmp_cmp2(|a, b| a.int() != b.int()),
                IF_ICMPLT => self.jmp_cmp2(|a, b| a.int() < b.int()),
                IF_ICMPGE => self.jmp_cmp2(|a, b| a.int() >= b.int()),
                IF_ICMPGT => self.jmp_cmp2(|a, b| a.int() > b.int()),
                IF_ICMPLE => self.jmp_cmp2(|a, b| a.int() <= b.int()),
                GOTO => self.jmp(),
                IRETURN | LRETURN | FRETURN | DRETURN => {
                    let return_address = self.frame_stack.last().unwrap().return_address;
                    let return_value = self.pop();

                    self.frame_stack.pop();
                    if self.frame_stack.is_empty() {
                        return return_value;
                    }

                    self.push(return_value);
                    self.pc = return_address;
                }
                RETURN => {
                    let return_address = self.frame_stack.last().unwrap().return_address;

                    self.frame_stack.pop();
                    if self.frame_stack.is_empty() {
                        return Void;
                    }

                    self.pc = return_address;
                }
                INVOKESTATIC => {
                    let index = self.read_u16();
                    let (class_name, name_and_type) = self.constant_pool().method_ref(index);

                    let class = &self.registry.class(class_name.as_str());
                    let method = class.method(name_and_type.as_str()).unwrap();

                    if method.is_native() {
                        let full_name = class_name + "." + &*name_and_type;
                        let native_method = self.registry.native_method(full_name.as_str());

                        let args = self.pop_args(&*method.descriptor);
                        match native_method(&args) {
                            Void => (),
                            return_value => self.push(return_value),
                        }

                        continue;
                    }

                    let mut frame = Frame::from(method, &class.constant_pool);
                    self.pop_args(&*method.descriptor)
                        .iter()
                        .enumerate()
                        .for_each(|(i, arg)| frame.locals[i] = *arg);
                    frame.return_address = self.pc;

                    self.frame_stack.push(frame);
                    self.pc = 0;
                }
                op => unimplemented!("instruction {:#x} is not supported", op),
            }
        }
    }

    fn cast<F>(&mut self, cast_fn: F)
    where
        F: FnOnce(Type) -> Type,
    {
        let x = self.pop();
        self.push(cast_fn(x))
    }

    fn fn2<F>(&mut self, op: F)
    where
        F: FnOnce(Type, Type) -> Type,
    {
        let (lhs, rhs) = self.pop2();
        self.push(op(lhs, rhs))
    }

    fn jmp(&mut self) {
        let (from, to) = (self.pc, self.pc + 2);
        let bytes = self.code()[from..to].try_into().unwrap();
        let address = u16::from_be_bytes(bytes);
        self.pc += address as usize - 1
    }

    fn jmp_cmp1<F>(&mut self, cond_fn: F)
    where
        F: FnOnce(Type) -> bool,
    {
        if cond_fn(self.pop()) {
            self.jmp()
        } else {
            self.read_u16();
        }
    }

    fn jmp_cmp2<F>(&mut self, cond_fn: F)
    where
        F: FnOnce(Type, Type) -> bool,
    {
        let (lhs, rhs) = self.pop2();
        if cond_fn(lhs, rhs) {
            self.jmp()
        } else {
            self.read_u16();
        }
    }

    fn load_n(&mut self, n: usize) {
        let variable = self.locals()[n];
        self.push(variable)
    }

    fn pop(&mut self) -> Type {
        self.operand_stack().pop().unwrap()
    }

    fn pop2(&mut self) -> (Type, Type) {
        let rhs = self.pop();
        let lhs = self.pop();
        (lhs, rhs)
    }

    fn push(&mut self, value: Type) {
        self.operand_stack().push(value)
    }

    fn read_u8(&mut self) -> u8 {
        let index = self.pc;
        self.pc += 1;
        self.code()[index]
    }

    fn read_u16(&mut self) -> u16 {
        let (from, to) = (self.pc, self.pc + 2);
        self.pc += 2;
        u16::from_be_bytes(self.code()[from..to].try_into().unwrap())
    }

    fn code(&self) -> &Vec<u8> {
        self.frame_stack.last().unwrap().code
    }

    fn constant_pool(&self) -> &ConstantPool {
        self.frame_stack.last().unwrap().constant_pool
    }

    fn locals(&mut self) -> &mut Vec<Type> {
        &mut self.frame_stack.last_mut().unwrap().locals
    }

    fn operand_stack(&mut self) -> &mut Vec<Type> {
        &mut self.frame_stack.last_mut().unwrap().operand_stack
    }

    pub fn new<'a>(
        class_name: &'a str,
        name_and_type: &'a str,
        registry: &'a Registry,
    ) -> Thread<'a> {
        let class = registry.class(class_name);
        let method = class.method(name_and_type).unwrap();
        let constant_pool = &class.constant_pool;

        Thread {
            frame_stack: vec![Frame::from(method, constant_pool)],
            pc: 0,
            registry,
        }
    }

    fn pop_args(&mut self, descriptor: &str) -> Vec<Type> {
        let (args_types, _) = descriptor.split_once(")").unwrap();
        let mut args = vec![];
        let re = Regex::new(r"(I)").unwrap();

        for cap in re.captures_iter(args_types) {
            match &cap[0] {
                "I" => {
                    let arg = self.pop();
                    matches!(arg, Type::Int(_));
                    args.push(arg);
                }
                _ => {}
            }
        }

        args.reverse();
        args
    }
}
