pub fn code() {
    println!("Hello from dep_402");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_402");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_402: {t}");
}

pub fn foo() {
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_104::code();
    dep_104::code_inlined();
    dep_104::code_generic(1u32);
    dep_97::code();
    dep_97::code_inlined();
    dep_97::code_generic(1u32);
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_123::code();
    dep_123::code_inlined();
    dep_123::code_generic(1u32);
}
