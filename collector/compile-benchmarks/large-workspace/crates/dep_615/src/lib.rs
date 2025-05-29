pub fn code() {
    println!("Hello from dep_615");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_615");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_615: {t}");
}

pub fn foo() {
    dep_399::code();
    dep_399::code_inlined();
    dep_399::code_generic(1u32);
}
