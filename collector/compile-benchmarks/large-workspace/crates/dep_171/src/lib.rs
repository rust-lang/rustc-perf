pub fn code() {
    println!("Hello from dep_171");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_171");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_171: {t}");
}

pub fn foo() {
    dep_85::code();
    dep_85::code_inlined();
    dep_85::code_generic(1u32);
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
}
