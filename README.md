# `build_const`: crate for creating constants in your build script

This is a crate for easily creating constants in your build script.
Simply write a `build.rs` like:
```
extern crate build_static;

fn create_constants() {
    let mut consts = build_static::ConstWriter::new("constants")
        .unwrap()
        .finish_dependencies();

    let values: Vec<u8> = vec![1, 2, 3, 36];

    // Add a single value as a result of a calculation
    consts.add_value("VALUE", "u8", values.iter().sum::<u8>());

    // Add a sized array
    consts.add_array("ARRAY", "u8", &values);

    let strs = vec!["foo", "bar", "baz"];

    // Add strings
    consts.add_array("STRS", "&str", &strs);

    // Add strings with some formatting
    let strs2: Vec<String> = strs
        .iter()
        .enumerate()
        .map(|(i, s)| format!("{}: {}", i, s)).collect();
    consts.add_array("STRS2", "&str", &strs2);

    // flush the file and finish
    consts.finish();
}

fn main() {
    create_constants();
}


```

Then use the library like:
```
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

fn main() {
    println!("VALUE: {}", VALUE);
    println!("VALUE: {}", ARRAY);
    println!("VALUE: {}", STRS);
    println!("VALUE: {}", STRS2);
}

```

See the `test_crates/` dir for more examples.
