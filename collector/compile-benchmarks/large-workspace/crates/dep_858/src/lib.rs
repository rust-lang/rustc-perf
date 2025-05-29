pub fn code() {
    println!("Hello from dep_858");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_858");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_858: {t}");
}

pub fn foo() {
    dep_171::code();
    dep_171::code_inlined();
    dep_171::code_generic(1u32);
}
