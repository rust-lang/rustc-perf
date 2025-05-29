pub fn code() {
    println!("Hello from dep_560");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_560");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_560: {t}");
}

pub fn foo() {
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
}
