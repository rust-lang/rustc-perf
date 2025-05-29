pub fn code() {
    println!("Hello from dep_185");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_185");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_185: {t}");
}

pub fn foo() {
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
    dep_116::code();
    dep_116::code_inlined();
    dep_116::code_generic(1u32);
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
}
