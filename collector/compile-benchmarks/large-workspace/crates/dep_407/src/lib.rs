pub fn code() {
    println!("Hello from dep_407");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_407");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_407: {t}");
}

pub fn foo() {
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_138::code();
    dep_138::code_inlined();
    dep_138::code_generic(1u32);
}
