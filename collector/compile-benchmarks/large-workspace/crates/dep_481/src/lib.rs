pub fn code() {
    println!("Hello from dep_481");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_481");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_481: {t}");
}

pub fn foo() {
    dep_360::code();
    dep_360::code_inlined();
    dep_360::code_generic(1u32);
}
