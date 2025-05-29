pub fn code() {
    println!("Hello from dep_12");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_12");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_12: {t}");
}

pub fn foo() {
    dep_9::code();
    dep_9::code_inlined();
    dep_9::code_generic(1u32);
    dep_8::code();
    dep_8::code_inlined();
    dep_8::code_generic(1u32);
    dep_5::code();
    dep_5::code_inlined();
    dep_5::code_generic(1u32);
}
