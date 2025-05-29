pub fn code() {
    println!("Hello from dep_120");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_120");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_120: {t}");
}

pub fn foo() {
    dep_16::code();
    dep_16::code_inlined();
    dep_16::code_generic(1u32);
}
