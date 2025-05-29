pub fn code() {
    println!("Hello from dep_207");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_207");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_207: {t}");
}

pub fn foo() {
    dep_138::code();
    dep_138::code_inlined();
    dep_138::code_generic(1u32);
}
