pub fn code() {
    println!("Hello from dep_846");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_846");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_846: {t}");
}

pub fn foo() {
    dep_290::code();
    dep_290::code_inlined();
    dep_290::code_generic(1u32);
    dep_397::code();
    dep_397::code_inlined();
    dep_397::code_generic(1u32);
    dep_254::code();
    dep_254::code_inlined();
    dep_254::code_generic(1u32);
    dep_355::code();
    dep_355::code_inlined();
    dep_355::code_generic(1u32);
}
