pub fn code() {
    println!("Hello from dep_780");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_780");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_780: {t}");
}

pub fn foo() {
    dep_385::code();
    dep_385::code_inlined();
    dep_385::code_generic(1u32);
}
