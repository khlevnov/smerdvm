use crate::class_loader::ClassLoader;
use crate::r#type::Type;
use crate::registry::Registry;
use crate::thread::Thread;

pub struct VirtualMachine {
    class_loader: ClassLoader,
    main_class: String,
    registry: Registry,
    initialization_queue: Vec<String>,
}

impl VirtualMachine {
    pub fn new(class_path: &str, main_class: &str) -> VirtualMachine {
        VirtualMachine {
            class_loader: ClassLoader::new(class_path),
            main_class: main_class.replace(".", "/"),
            registry: Registry::default(),
            initialization_queue: vec![],
        }
    }

    pub fn run(&mut self, _args: &[&str]) {
        self.registry
            .add_native("ru/khlevnov/PrintStream.print:(I)V", print_int)
            .add_native("ru/khlevnov/PrintStream.print:(D)V", print_double);

        self.eager_load_recursively(&self.main_class.to_owned());
        self.initialize();

        let thread = Thread::new(
            "ru/khlevnov/Main",
            "main:([Ljava/lang/String;)V",
            &self.registry,
        );
        thread.run();
    }

    fn eager_load_recursively(&mut self, class_name: &str) {
        if class_name == "java/lang/Object" {
            return; // TODO should be loaded from rt.jar before
        }

        if self.registry.classes.contains_key(class_name) {
            return; // TODO hide into registry
        }

        let class = self.class_loader.find_class(class_name);
        let class_name = class.this_class();

        self.eager_load_recursively(&*class.super_class());
        self.registry.add_class(class);
        self.initialization_queue.push(class_name.clone());

        let class = self.registry.class(&*class_name);
        let classes_names = class.constant_pool.classes();

        for class_name in classes_names {
            self.eager_load_recursively(&*class_name);
        }
    }

    pub fn initialize(&mut self) {
        for class_name in &self.initialization_queue {
            if let Some(_) = self.registry.class(class_name).method("<clinit>:()V") {
                Thread::new(class_name, "<clinit>:()V", &self.registry).run();
            }
        }
    }
}

fn print_int(args: &[Type]) -> Type {
    println!("{}", args[0].int());
    return Type::Void;
}

fn print_double(args: &[Type]) -> Type {
    println!("{}", args[0].double());
    return Type::Void;
}
