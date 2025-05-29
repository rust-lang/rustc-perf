pub fn code() {
    println!("Hello from dep_249");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_249");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_249: {t}");
}

pub fn foo() {
    dep_152::code();
    dep_152::code_inlined();
    dep_152::code_generic(1u32);
    dep_148::code();
    dep_148::code_inlined();
    dep_148::code_generic(1u32);
}
