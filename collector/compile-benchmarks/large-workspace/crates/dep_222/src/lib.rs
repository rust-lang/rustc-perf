pub fn code() {
    println!("Hello from dep_222");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_222");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_222: {t}");
}

pub fn foo() {
    dep_66::code();
    dep_66::code_inlined();
    dep_66::code_generic(1u32);
    dep_145::code();
    dep_145::code_inlined();
    dep_145::code_generic(1u32);
}
