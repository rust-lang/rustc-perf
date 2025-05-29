pub fn code() {
    println!("Hello from dep_383");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_383");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_383: {t}");
}

pub fn foo() {
    dep_134::code();
    dep_134::code_inlined();
    dep_134::code_generic(1u32);
    dep_66::code();
    dep_66::code_inlined();
    dep_66::code_generic(1u32);
}
