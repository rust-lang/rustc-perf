pub fn code() {
    println!("Hello from dep_260");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_260");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_260: {t}");
}

pub fn foo() {
    dep_112::code();
    dep_112::code_inlined();
    dep_112::code_generic(1u32);
    dep_106::code();
    dep_106::code_inlined();
    dep_106::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
    dep_114::code();
    dep_114::code_inlined();
    dep_114::code_generic(1u32);
}
