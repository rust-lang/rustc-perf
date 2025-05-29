pub fn code() {
    println!("Hello from dep_271");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_271");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_271: {t}");
}

pub fn foo() {
    dep_137::code();
    dep_137::code_inlined();
    dep_137::code_generic(1u32);
    dep_104::code();
    dep_104::code_inlined();
    dep_104::code_generic(1u32);
    dep_152::code();
    dep_152::code_inlined();
    dep_152::code_generic(1u32);
    dep_145::code();
    dep_145::code_inlined();
    dep_145::code_generic(1u32);
}
