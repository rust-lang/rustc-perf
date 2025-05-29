pub fn code() {
    println!("Hello from dep_601");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_601");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_601: {t}");
}

pub fn foo() {
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
}
