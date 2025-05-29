pub fn code() {
    println!("Hello from dep_294");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_294");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_294: {t}");
}

pub fn foo() {
    dep_106::code();
    dep_106::code_inlined();
    dep_106::code_generic(1u32);
    dep_86::code();
    dep_86::code_inlined();
    dep_86::code_generic(1u32);
    dep_96::code();
    dep_96::code_inlined();
    dep_96::code_generic(1u32);
}
