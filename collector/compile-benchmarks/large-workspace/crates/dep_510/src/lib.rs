pub fn code() {
    println!("Hello from dep_510");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_510");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_510: {t}");
}

pub fn foo() {
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
}
