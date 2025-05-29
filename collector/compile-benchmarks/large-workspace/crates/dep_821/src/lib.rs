pub fn code() {
    println!("Hello from dep_821");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_821");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_821: {t}");
}

pub fn foo() {
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_242::code();
    dep_242::code_inlined();
    dep_242::code_generic(1u32);
}
