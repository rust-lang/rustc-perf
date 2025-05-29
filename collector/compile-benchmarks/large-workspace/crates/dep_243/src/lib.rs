pub fn code() {
    println!("Hello from dep_243");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_243");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_243: {t}");
}

pub fn foo() {
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
    dep_124::code();
    dep_124::code_inlined();
    dep_124::code_generic(1u32);
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
    dep_110::code();
    dep_110::code_inlined();
    dep_110::code_generic(1u32);
}
