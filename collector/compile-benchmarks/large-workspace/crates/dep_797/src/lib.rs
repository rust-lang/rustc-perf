pub fn code() {
    println!("Hello from dep_797");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_797");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_797: {t}");
}

pub fn foo() {
    dep_177::code();
    dep_177::code_inlined();
    dep_177::code_generic(1u32);
    dep_236::code();
    dep_236::code_inlined();
    dep_236::code_generic(1u32);
    dep_232::code();
    dep_232::code_inlined();
    dep_232::code_generic(1u32);
    dep_295::code();
    dep_295::code_inlined();
    dep_295::code_generic(1u32);
}
