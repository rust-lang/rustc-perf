pub fn code() {
    println!("Hello from dep_237");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_237");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_237: {t}");
}

pub fn foo() {
    dep_111::code();
    dep_111::code_inlined();
    dep_111::code_generic(1u32);
    dep_74::code();
    dep_74::code_inlined();
    dep_74::code_generic(1u32);
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
}
