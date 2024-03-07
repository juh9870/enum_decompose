use variant_extraction::extract_variants;

mod tests;

#[extract_variants]
#[derive(Debug, Clone)]
enum Test1 {
    A,
    B(Box<Test1>),
    C(usize, f64),
    D { field1: bool, field2: String },
}
