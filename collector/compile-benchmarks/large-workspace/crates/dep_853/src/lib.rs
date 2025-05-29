pub fn code() {
    println!("Hello from dep_853");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_853");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_853: {t}");
}

pub fn foo() {
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
}
