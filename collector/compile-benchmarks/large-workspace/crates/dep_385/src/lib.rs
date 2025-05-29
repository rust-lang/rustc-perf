pub fn code() {
    println!("Hello from dep_385");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_385");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_385: {t}");
}

pub fn foo() {
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
}
