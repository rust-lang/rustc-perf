pub fn code() {
    println!("Hello from dep_632");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_632");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_632: {t}");
}

pub fn foo() {
    dep_387::code();
    dep_387::code_inlined();
    dep_387::code_generic(1u32);
    dep_364::code();
    dep_364::code_inlined();
    dep_364::code_generic(1u32);
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
}
