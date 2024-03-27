`enum_decompose` provides a single macro to extract struct enum variants into separate types, leaving the original enum with tuples

# Practical example
No beating around the bush

```rust
use enum_decompose::decompose;

#[decompose]
#[derive(Debug, Clone)]
enum Value {
    Number,
    B {},
}
```

