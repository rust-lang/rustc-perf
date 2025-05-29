pub fn code() {
    println!("Hello from dep_909");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_909");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_909: {t}");
}

pub fn foo() {
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
}
