pub fn code() {
    println!("Hello from dep_167");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_167");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_167: {t}");
}

pub fn foo() {
    dep_157::code();
    dep_157::code_inlined();
    dep_157::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
}
