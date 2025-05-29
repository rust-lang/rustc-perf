pub fn code() {
    println!("Hello from dep_217");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_217");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_217: {t}");
}

pub fn foo() {
    dep_74::code();
    dep_74::code_inlined();
    dep_74::code_generic(1u32);
    dep_124::code();
    dep_124::code_inlined();
    dep_124::code_generic(1u32);
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
    dep_122::code();
    dep_122::code_inlined();
    dep_122::code_generic(1u32);
}
