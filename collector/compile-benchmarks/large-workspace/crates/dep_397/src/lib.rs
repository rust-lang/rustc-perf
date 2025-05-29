pub fn code() {
    println!("Hello from dep_397");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_397");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_397: {t}");
}

pub fn foo() {
    dep_155::code();
    dep_155::code_inlined();
    dep_155::code_generic(1u32);
    dep_82::code();
    dep_82::code_inlined();
    dep_82::code_generic(1u32);
}
