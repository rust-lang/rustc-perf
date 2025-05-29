pub fn code() {
    println!("Hello from dep_231");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_231");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_231: {t}");
}

pub fn foo() {
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
}
