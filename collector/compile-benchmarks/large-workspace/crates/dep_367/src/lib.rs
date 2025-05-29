pub fn code() {
    println!("Hello from dep_367");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_367");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_367: {t}");
}

pub fn foo() {
    dep_101::code();
    dep_101::code_inlined();
    dep_101::code_generic(1u32);
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_158::code();
    dep_158::code_inlined();
    dep_158::code_generic(1u32);
}
