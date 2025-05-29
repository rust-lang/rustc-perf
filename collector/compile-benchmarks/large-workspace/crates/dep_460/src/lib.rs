pub fn code() {
    println!("Hello from dep_460");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_460");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_460: {t}");
}

pub fn foo() {
    dep_176::code();
    dep_176::code_inlined();
    dep_176::code_generic(1u32);
    dep_279::code();
    dep_279::code_inlined();
    dep_279::code_generic(1u32);
    dep_190::code();
    dep_190::code_inlined();
    dep_190::code_generic(1u32);
    dep_319::code();
    dep_319::code_inlined();
    dep_319::code_generic(1u32);
}
