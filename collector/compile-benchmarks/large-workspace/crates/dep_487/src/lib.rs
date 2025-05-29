pub fn code() {
    println!("Hello from dep_487");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_487");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_487: {t}");
}

pub fn foo() {
    dep_279::code();
    dep_279::code_inlined();
    dep_279::code_generic(1u32);
    dep_270::code();
    dep_270::code_inlined();
    dep_270::code_generic(1u32);
    dep_227::code();
    dep_227::code_inlined();
    dep_227::code_generic(1u32);
    dep_220::code();
    dep_220::code_inlined();
    dep_220::code_generic(1u32);
}
