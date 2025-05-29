pub fn code() {
    println!("Hello from dep_181");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_181");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_181: {t}");
}

pub fn foo() {
    dep_123::code();
    dep_123::code_inlined();
    dep_123::code_generic(1u32);
    dep_126::code();
    dep_126::code_inlined();
    dep_126::code_generic(1u32);
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
}
