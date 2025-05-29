pub fn code() {
    println!("Hello from dep_483");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_483");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_483: {t}");
}

pub fn foo() {
    dep_365::code();
    dep_365::code_inlined();
    dep_365::code_generic(1u32);
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
    dep_255::code();
    dep_255::code_inlined();
    dep_255::code_generic(1u32);
    dep_190::code();
    dep_190::code_inlined();
    dep_190::code_generic(1u32);
}
