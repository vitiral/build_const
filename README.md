# `build_const`: crate for creating constants in your build script

This is a crate for easily creating constants in a script or `build.rs` file.

## Simple Example

**Cargo.toml**:
```
[package]
# ...
build = "build.rs"

[dependencies]
build_const = "0.1"

[build-dependencies]
build_const = "0.1"
```

**build.rs**:
```
extern crate build_const;

fn create_constants() {
    let mut consts = build_const::ConstWriter::for_build("constants")
        .unwrap()
        // you can write dependencies before `finish_dependencies`
        .finish_dependencies();

    // do some "complex" calculations
    let values: Vec<u8> = vec![1, 2, 3, 36];

    // add both the array and the calculations to your constants
    consts.add_value("VALUE", "u8", values.iter().sum::<u8>());
    consts.add_array("ARRAY", "u8", &values);
    consts.finish()
}
```

**main.rs**
```
#[macro_use]
extern crate build_const;

build_const!("constants.rs");

fn main() {
    println!("VALUE: {}", VALUE);
    println!("VALUE: {}", ARRAY);
}
```

## Using in a Script
Using in a script is much the same, except you should use
`ConstWriter::from_path` and give it the path to your constants file.

# License
The license is MIT. This is intended to be absolutely open source software and
useable for any application.
