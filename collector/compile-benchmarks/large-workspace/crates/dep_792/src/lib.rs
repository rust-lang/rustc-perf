pub fn code() {
    println!("Hello from dep_792");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_792");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_792: {t}");
}

pub fn foo() {
    dep_183::code();
    dep_183::code_inlined();
    dep_183::code_generic(1u32);
}
