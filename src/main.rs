use std::fs::File;
use crate::class_file::ClassFile;

mod class_file;

fn main() -> Result<(), std::io::Error> {
    let f = File::open("/Users/khlevnov/smerdvm/playground/app/\
    build/classes/java/main/ru/khlevnov/Empty.class")?;

    let class_file = ClassFile::from(f);
    println!("{:?}", class_file);

    Ok(())
}
