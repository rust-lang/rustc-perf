pub fn code() {
    println!("Hello from dep_779");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_779");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_779: {t}");
}

pub fn foo() {
    dep_337::code();
    dep_337::code_inlined();
    dep_337::code_generic(1u32);
    dep_383::code();
    dep_383::code_inlined();
    dep_383::code_generic(1u32);
}
