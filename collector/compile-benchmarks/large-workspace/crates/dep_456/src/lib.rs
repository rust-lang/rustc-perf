pub fn code() {
    println!("Hello from dep_456");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_456");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_456: {t}");
}

pub fn foo() {
    dep_397::code();
    dep_397::code_inlined();
    dep_397::code_generic(1u32);
    dep_228::code();
    dep_228::code_inlined();
    dep_228::code_generic(1u32);
    dep_306::code();
    dep_306::code_inlined();
    dep_306::code_generic(1u32);
}
