pub fn code() {
    println!("Hello from dep_869");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_869");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_869: {t}");
}

pub fn foo() {
    dep_260::code();
    dep_260::code_inlined();
    dep_260::code_generic(1u32);
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
    dep_330::code();
    dep_330::code_inlined();
    dep_330::code_generic(1u32);
    dep_353::code();
    dep_353::code_inlined();
    dep_353::code_generic(1u32);
    dep_306::code();
    dep_306::code_inlined();
    dep_306::code_generic(1u32);
}
