pub fn code() {
    println!("Hello from dep_233");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_233");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_233: {t}");
}

pub fn foo() {
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_101::code();
    dep_101::code_inlined();
    dep_101::code_generic(1u32);
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
}
