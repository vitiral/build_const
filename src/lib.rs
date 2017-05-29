use std::env;
use std::fs;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::path::Path;
use std::str;


pub struct ConstWriter {
    f: fs::File,
}

/// the writer object for specifying constants.
pub struct ConstValueWriter {
    f: fs::File,
}

impl ConstWriter {
    /// Get a new dependency writer. Use this to write your dependencies
    /// (`use mylib` statements) and then call `finish` to write your
    /// actual constants.
    pub fn new(mod_name: &str) -> io::Result<ConstWriter> {
        let out_dir = env::var("OUT_DIR").unwrap();
        let mod_name = format!("{}.rs", mod_name);
        let dest_path = Path::new(&out_dir).join(mod_name);

        Ok(ConstWriter {
            f: fs::File::create(&dest_path)?
        })
    }

    /// Add a dependency to your constants file.
    pub fn add_dependency(&mut self, lib: &str) {
        write!(self.f, "pub use {};\n", lib).unwrap();
    }

    /// finish writing dependencies and start writing constants
    pub fn finish_dependencies(self) -> ConstValueWriter {
        ConstValueWriter { f: self.f }
    }

}

impl ConstValueWriter {
    /// Add a value to your declared constants string.
    /// 
    /// You have to manually specify the `name`, type (`ty`) and `value`
    /// of the constant you want to add and it will be added.
    /// 
    /// You can use a crate like t_bang to make this more ergonomic.
    pub fn add_value<T: Debug>(&mut self, name: &str, ty: &str, value: T) {
        let value_str = format!("{:?}", value);
        write!(
            self.f, "pub const {}: {} = {};\n", 
            name, 
            ty,
            value_str
        ).unwrap();
    }

    pub fn add_array<T: Debug>(&mut self, name: &str, ty: &str, value: &[T]) {
        let value_str = format!("{:?}", value);

        write!(
            self.f, "pub const {}: [{}; {}] = {};\n", 
            name, 
            ty,
            value.len(),
            value_str,
        ).unwrap();
    }

    pub fn finish(&mut self) {
        self.f.flush().unwrap();
    }
}

#[test]
fn test_basic() {
    //let mut data: Vec<u8> = Vec::new();
    //{
    //    let mut w = io::BufWriter::new(&mut data);
    //    let mut dep_writer = ConstWriter::new("fake", &mut w);
    //    dep_writer.add_dependency("foobar");
    //    let mut value_writer = dep_writer.finish();
    //    value_writer.add_value("foo", "foobar::Foo", "Foo { bar: 7 }");
    //}
    //assert_eq!("pub use foobar;\n\
    //    pub const foo: foobar::Foo = Foo { bar: 7 };\n",
    //    str::from_utf8(&data).unwrap()
    //);
}
