pub fn code() {
    println!("Hello from dep_79");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_79");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_79: {t}");
}

pub fn foo() {
    dep_26::code();
    dep_26::code_inlined();
    dep_26::code_generic(1u32);
}
