pub fn code() {
    println!("Hello from dep_886");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_886");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_886: {t}");
}

pub fn foo() {
    dep_191::code();
    dep_191::code_inlined();
    dep_191::code_generic(1u32);
}
