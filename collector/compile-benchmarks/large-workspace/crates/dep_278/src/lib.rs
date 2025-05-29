pub fn code() {
    println!("Hello from dep_278");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_278");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_278: {t}");
}

pub fn foo() {
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
}
