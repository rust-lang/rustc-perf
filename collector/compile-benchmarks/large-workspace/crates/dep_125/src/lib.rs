pub fn code() {
    println!("Hello from dep_125");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_125");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_125: {t}");
}

pub fn foo() {
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
    dep_12::code();
    dep_12::code_inlined();
    dep_12::code_generic(1u32);
}
