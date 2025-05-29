pub fn code() {
    println!("Hello from dep_463");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_463");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_463: {t}");
}

pub fn foo() {
    dep_285::code();
    dep_285::code_inlined();
    dep_285::code_generic(1u32);
    dep_190::code();
    dep_190::code_inlined();
    dep_190::code_generic(1u32);
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
    dep_242::code();
    dep_242::code_inlined();
    dep_242::code_generic(1u32);
}
