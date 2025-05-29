pub fn code() {
    println!("Hello from dep_155");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_155");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_155: {t}");
}

pub fn foo() {
    dep_12::code();
    dep_12::code_inlined();
    dep_12::code_generic(1u32);
}
