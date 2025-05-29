pub fn code() {
    println!("Hello from dep_203");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_203");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_203: {t}");
}

pub fn foo() {
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
    dep_87::code();
    dep_87::code_inlined();
    dep_87::code_generic(1u32);
}
