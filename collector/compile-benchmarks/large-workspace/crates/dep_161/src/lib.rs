pub fn code() {
    println!("Hello from dep_161");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_161");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_161: {t}");
}

pub fn foo() {
    dep_71::code();
    dep_71::code_inlined();
    dep_71::code_generic(1u32);
}
