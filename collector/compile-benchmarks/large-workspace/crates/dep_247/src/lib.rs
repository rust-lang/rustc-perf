pub fn code() {
    println!("Hello from dep_247");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_247");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_247: {t}");
}

pub fn foo() {
    dep_129::code();
    dep_129::code_inlined();
    dep_129::code_generic(1u32);
}
