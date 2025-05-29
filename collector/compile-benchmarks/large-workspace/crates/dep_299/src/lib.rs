pub fn code() {
    println!("Hello from dep_299");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_299");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_299: {t}");
}

pub fn foo() {
    dep_83::code();
    dep_83::code_inlined();
    dep_83::code_generic(1u32);
    dep_99::code();
    dep_99::code_inlined();
    dep_99::code_generic(1u32);
    dep_158::code();
    dep_158::code_inlined();
    dep_158::code_generic(1u32);
    dep_60::code();
    dep_60::code_inlined();
    dep_60::code_generic(1u32);
}
