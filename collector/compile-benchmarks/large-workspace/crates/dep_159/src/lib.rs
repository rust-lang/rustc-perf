pub fn code() {
    println!("Hello from dep_159");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_159");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_159: {t}");
}

pub fn foo() {
    dep_20::code();
    dep_20::code_inlined();
    dep_20::code_generic(1u32);
    dep_16::code();
    dep_16::code_inlined();
    dep_16::code_generic(1u32);
}
