# fusion

## Description
Merge two instances of the same type by replacing fields in the source instance with
the fields containing a value in the second instance (fields set to `Some(thing)` or fields not wrapped in an `Option`).

Fields set to `None` in the second instance are left untouched in the source instance.

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
