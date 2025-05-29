pub fn code() {
    println!("Hello from dep_851");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_851");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_851: {t}");
}

pub fn foo() {
    dep_295::code();
    dep_295::code_inlined();
    dep_295::code_generic(1u32);
    dep_270::code();
    dep_270::code_inlined();
    dep_270::code_generic(1u32);
}
