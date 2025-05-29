pub fn code() {
    println!("Hello from dep_710");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_710");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_710: {t}");
}

pub fn foo() {
    dep_319::code();
    dep_319::code_inlined();
    dep_319::code_generic(1u32);
    dep_351::code();
    dep_351::code_inlined();
    dep_351::code_generic(1u32);
    dep_377::code();
    dep_377::code_inlined();
    dep_377::code_generic(1u32);
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_322::code();
    dep_322::code_inlined();
    dep_322::code_generic(1u32);
}
