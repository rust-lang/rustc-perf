pub fn code() {
    println!("Hello from dep_416");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_416");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_416: {t}");
}

pub fn foo() {
    dep_365::code();
    dep_365::code_inlined();
    dep_365::code_generic(1u32);
    dep_385::code();
    dep_385::code_inlined();
    dep_385::code_generic(1u32);
    dep_171::code();
    dep_171::code_inlined();
    dep_171::code_generic(1u32);
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
}
