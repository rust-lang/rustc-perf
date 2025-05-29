pub fn code() {
    println!("Hello from dep_200");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_200");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_200: {t}");
}

pub fn foo() {
    dep_64::code();
    dep_64::code_inlined();
    dep_64::code_generic(1u32);
    dep_123::code();
    dep_123::code_inlined();
    dep_123::code_generic(1u32);
}
