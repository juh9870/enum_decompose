use enum_decompose::decompose;

mod tests;

#[decompose]
#[derive(Debug, Clone)]
enum Test1 {
    A,
    B(Box<Test1>),
    C(usize, f64),
    D { field1: bool, field2: String },
}
