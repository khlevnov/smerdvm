use crate::vm::VirtualMachine;

mod class;
mod class_file;
mod class_loader;
mod constant_pool;
mod frame;
mod method;
mod registry;
mod thread;
mod r#type;
mod vm;

fn main() {
    let class_path = "/Users/khlevnov/smerdvm/playground/app/build/classes/java/main";
    let main_class = "ru.khlevnov.Main";

    let mut vm = VirtualMachine::new(class_path, &*main_class);
    vm.run(&[]);
}
