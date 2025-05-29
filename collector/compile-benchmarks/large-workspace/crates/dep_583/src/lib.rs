pub fn code() {
    println!("Hello from dep_583");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_583");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_583: {t}");
}

pub fn foo() {
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
    dep_355::code();
    dep_355::code_inlined();
    dep_355::code_generic(1u32);
}
