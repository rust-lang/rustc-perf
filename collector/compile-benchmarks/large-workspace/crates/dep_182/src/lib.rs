pub fn code() {
    println!("Hello from dep_182");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_182");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_182: {t}");
}

pub fn foo() {
    dep_123::code();
    dep_123::code_inlined();
    dep_123::code_generic(1u32);
}
