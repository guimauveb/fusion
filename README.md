# fusion

## Description
Easy to use proc-macro to "merge" two instances of the same type.

I came accross a few cases where I needed to merge two instances of the same type, or more specifically, replace the fields in a source instance with the fields containing a value in a second instance (fields set to `Some(thing)` or fields not wrapped in an `Option`).

Fields set to `None` in the second instance are left untouched in the source instance.

## Usage

Add the `Fusion` derive macro to a struct.

The `#[fusion]` attribute is used when the field type implements `Fusion` and that it should
be called when merging the parent struct.

## Example
```rust
#[derive(Debug, PartialEq, Eq, Fusion)]
struct Foo {
    a: Option<String>,
    b: Option<usize>,
    c: String,
}

let mut src = Foo {
    a: Some("Bar".into()),
    b: Some(7),
    c: "One".into(),
};
let update = Foo {
    a: None,
    b: Some(8),
    c: "Two".into(),
};
src.merge(update);
assert_eq!(
    src,
    Foo {
        a: Some("Bar".into()),
        b: Some(8),
        c: "Two".into(),
    }
);
```

