

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

fn main() {
    println!("VALUE: {}", VALUE);
}

#[test]
fn test_build() {
    assert_eq!(VALUE, 42);
    assert_eq!(ARRAY, [1, 2, 3, 36]);
    assert_eq!(ARRAY, [1, 2, 3, 36]);

    let strs = ["foo", "bar", "baz"];
    let strs2 = ["0: foo", "1: bar", "2: baz"];
    assert_eq!(STRS, strs);
    assert_eq!(STRS2, strs2);
}
