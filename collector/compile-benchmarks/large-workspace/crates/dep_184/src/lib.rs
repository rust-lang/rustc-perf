pub fn code() {
    println!("Hello from dep_184");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_184");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_184: {t}");
}

pub fn foo() {
    dep_116::code();
    dep_116::code_inlined();
    dep_116::code_generic(1u32);
}
