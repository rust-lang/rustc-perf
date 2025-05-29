pub fn code() {
    println!("Hello from dep_282");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_282");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_282: {t}");
}

pub fn foo() {
    dep_142::code();
    dep_142::code_inlined();
    dep_142::code_generic(1u32);
}
