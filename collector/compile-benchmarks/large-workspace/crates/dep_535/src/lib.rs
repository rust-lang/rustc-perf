pub fn code() {
    println!("Hello from dep_535");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_535");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_535: {t}");
}

pub fn foo() {
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
    dep_223::code();
    dep_223::code_inlined();
    dep_223::code_generic(1u32);
    dep_256::code();
    dep_256::code_inlined();
    dep_256::code_generic(1u32);
}
