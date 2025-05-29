pub fn code() {
    println!("Hello from dep_428");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_428");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_428: {t}");
}

pub fn foo() {
    dep_320::code();
    dep_320::code_inlined();
    dep_320::code_generic(1u32);
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
    dep_403::code();
    dep_403::code_inlined();
    dep_403::code_generic(1u32);
    dep_252::code();
    dep_252::code_inlined();
    dep_252::code_generic(1u32);
}
