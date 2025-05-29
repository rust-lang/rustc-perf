pub fn code() {
    println!("Hello from dep_251");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_251");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_251: {t}");
}

pub fn foo() {
    dep_64::code();
    dep_64::code_inlined();
    dep_64::code_generic(1u32);
    dep_149::code();
    dep_149::code_inlined();
    dep_149::code_generic(1u32);
}
