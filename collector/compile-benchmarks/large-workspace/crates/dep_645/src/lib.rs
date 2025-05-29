pub fn code() {
    println!("Hello from dep_645");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_645");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_645: {t}");
}

pub fn foo() {
    dep_252::code();
    dep_252::code_inlined();
    dep_252::code_generic(1u32);
}
