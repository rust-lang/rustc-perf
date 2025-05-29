pub fn code() {
    println!("Hello from dep_419");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_419");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_419: {t}");
}

pub fn foo() {
    dep_383::code();
    dep_383::code_inlined();
    dep_383::code_generic(1u32);
    dep_260::code();
    dep_260::code_inlined();
    dep_260::code_generic(1u32);
    dep_376::code();
    dep_376::code_inlined();
    dep_376::code_generic(1u32);
    dep_400::code();
    dep_400::code_inlined();
    dep_400::code_generic(1u32);
    dep_220::code();
    dep_220::code_inlined();
    dep_220::code_generic(1u32);
}
