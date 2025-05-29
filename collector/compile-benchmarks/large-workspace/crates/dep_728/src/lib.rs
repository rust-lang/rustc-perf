pub fn code() {
    println!("Hello from dep_728");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_728");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_728: {t}");
}

pub fn foo() {
    dep_242::code();
    dep_242::code_inlined();
    dep_242::code_generic(1u32);
}
