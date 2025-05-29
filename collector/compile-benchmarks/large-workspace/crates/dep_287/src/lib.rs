pub fn code() {
    println!("Hello from dep_287");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_287");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_287: {t}");
}

pub fn foo() {
    dep_131::code();
    dep_131::code_inlined();
    dep_131::code_generic(1u32);
    dep_153::code();
    dep_153::code_inlined();
    dep_153::code_generic(1u32);
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
    dep_145::code();
    dep_145::code_inlined();
    dep_145::code_generic(1u32);
}
