pub fn code() {
    println!("Hello from dep_831");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_831");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_831: {t}");
}

pub fn foo() {
    dep_273::code();
    dep_273::code_inlined();
    dep_273::code_generic(1u32);
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
    dep_206::code();
    dep_206::code_inlined();
    dep_206::code_generic(1u32);
    dep_392::code();
    dep_392::code_inlined();
    dep_392::code_generic(1u32);
    dep_198::code();
    dep_198::code_inlined();
    dep_198::code_generic(1u32);
}
