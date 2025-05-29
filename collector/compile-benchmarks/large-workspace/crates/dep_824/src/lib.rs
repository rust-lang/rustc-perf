pub fn code() {
    println!("Hello from dep_824");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_824");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_824: {t}");
}

pub fn foo() {
    dep_337::code();
    dep_337::code_inlined();
    dep_337::code_generic(1u32);
    dep_221::code();
    dep_221::code_inlined();
    dep_221::code_generic(1u32);
    dep_327::code();
    dep_327::code_inlined();
    dep_327::code_generic(1u32);
    dep_260::code();
    dep_260::code_inlined();
    dep_260::code_generic(1u32);
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
}
