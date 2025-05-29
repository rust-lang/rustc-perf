pub fn code() {
    println!("Hello from dep_365");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_365");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_365: {t}");
}

pub fn foo() {
    dep_145::code();
    dep_145::code_inlined();
    dep_145::code_generic(1u32);
    dep_149::code();
    dep_149::code_inlined();
    dep_149::code_generic(1u32);
    dep_79::code();
    dep_79::code_inlined();
    dep_79::code_generic(1u32);
}
