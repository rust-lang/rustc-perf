pub fn code() {
    println!("Hello from dep_459");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_459");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_459: {t}");
}

pub fn foo() {
    dep_394::code();
    dep_394::code_inlined();
    dep_394::code_generic(1u32);
}
