pub fn code() {
    println!("Hello from dep_789");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_789");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_789: {t}");
}

pub fn foo() {
    dep_207::code();
    dep_207::code_inlined();
    dep_207::code_generic(1u32);
    dep_227::code();
    dep_227::code_inlined();
    dep_227::code_generic(1u32);
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
}
