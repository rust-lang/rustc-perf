pub fn code() {
    println!("Hello from dep_178");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_178");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_178: {t}");
}

pub fn foo() {
    dep_152::code();
    dep_152::code_inlined();
    dep_152::code_generic(1u32);
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
    dep_153::code();
    dep_153::code_inlined();
    dep_153::code_generic(1u32);
    dep_137::code();
    dep_137::code_inlined();
    dep_137::code_generic(1u32);
    dep_101::code();
    dep_101::code_inlined();
    dep_101::code_generic(1u32);
}
