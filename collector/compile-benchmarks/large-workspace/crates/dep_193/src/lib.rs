pub fn code() {
    println!("Hello from dep_193");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_193");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_193: {t}");
}

pub fn foo() {
    dep_102::code();
    dep_102::code_inlined();
    dep_102::code_generic(1u32);
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
}
