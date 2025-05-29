pub fn code() {
    println!("Hello from dep_313");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_313");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_313: {t}");
}

pub fn foo() {
    dep_60::code();
    dep_60::code_inlined();
    dep_60::code_generic(1u32);
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
    dep_110::code();
    dep_110::code_inlined();
    dep_110::code_generic(1u32);
}
