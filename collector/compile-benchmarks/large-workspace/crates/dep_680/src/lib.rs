pub fn code() {
    println!("Hello from dep_680");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_680");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_680: {t}");
}

pub fn foo() {
    dep_349::code();
    dep_349::code_inlined();
    dep_349::code_generic(1u32);
    dep_275::code();
    dep_275::code_inlined();
    dep_275::code_generic(1u32);
}
