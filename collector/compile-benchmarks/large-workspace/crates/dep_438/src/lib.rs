pub fn code() {
    println!("Hello from dep_438");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_438");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_438: {t}");
}

pub fn foo() {
    dep_228::code();
    dep_228::code_inlined();
    dep_228::code_generic(1u32);
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
    dep_171::code();
    dep_171::code_inlined();
    dep_171::code_generic(1u32);
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
}
