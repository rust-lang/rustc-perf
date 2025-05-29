pub fn code() {
    println!("Hello from dep_514");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_514");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_514: {t}");
}

pub fn foo() {
    dep_275::code();
    dep_275::code_inlined();
    dep_275::code_generic(1u32);
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
    dep_341::code();
    dep_341::code_inlined();
    dep_341::code_generic(1u32);
    dep_236::code();
    dep_236::code_inlined();
    dep_236::code_generic(1u32);
}
