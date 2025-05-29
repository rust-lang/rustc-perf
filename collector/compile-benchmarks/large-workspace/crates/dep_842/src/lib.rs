pub fn code() {
    println!("Hello from dep_842");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_842");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_842: {t}");
}

pub fn foo() {
    dep_330::code();
    dep_330::code_inlined();
    dep_330::code_generic(1u32);
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
}
