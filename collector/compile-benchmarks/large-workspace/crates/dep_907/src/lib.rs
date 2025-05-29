pub fn code() {
    println!("Hello from dep_907");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_907");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_907: {t}");
}

pub fn foo() {
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
    dep_343::code();
    dep_343::code_inlined();
    dep_343::code_generic(1u32);
}
