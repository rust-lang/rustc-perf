pub fn code() {
    println!("Hello from dep_432");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_432");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_432: {t}");
}

pub fn foo() {
    dep_307::code();
    dep_307::code_inlined();
    dep_307::code_generic(1u32);
    dep_319::code();
    dep_319::code_inlined();
    dep_319::code_generic(1u32);
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
}
