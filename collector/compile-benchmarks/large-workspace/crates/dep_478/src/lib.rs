pub fn code() {
    println!("Hello from dep_478");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_478");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_478: {t}");
}

pub fn foo() {
    dep_236::code();
    dep_236::code_inlined();
    dep_236::code_generic(1u32);
    dep_337::code();
    dep_337::code_inlined();
    dep_337::code_generic(1u32);
    dep_240::code();
    dep_240::code_inlined();
    dep_240::code_generic(1u32);
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
}
