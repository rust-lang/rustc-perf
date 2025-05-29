pub fn code() {
    println!("Hello from dep_225");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_225");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_225: {t}");
}

pub fn foo() {
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_61::code();
    dep_61::code_inlined();
    dep_61::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_108::code();
    dep_108::code_inlined();
    dep_108::code_generic(1u32);
}
