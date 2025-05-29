pub fn code() {
    println!("Hello from dep_325");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_325");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_325: {t}");
}

pub fn foo() {
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_106::code();
    dep_106::code_inlined();
    dep_106::code_generic(1u32);
    dep_123::code();
    dep_123::code_inlined();
    dep_123::code_generic(1u32);
    dep_80::code();
    dep_80::code_inlined();
    dep_80::code_generic(1u32);
}
