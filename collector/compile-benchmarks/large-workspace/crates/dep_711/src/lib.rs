pub fn code() {
    println!("Hello from dep_711");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_711");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_711: {t}");
}

pub fn foo() {
    dep_258::code();
    dep_258::code_inlined();
    dep_258::code_generic(1u32);
}
