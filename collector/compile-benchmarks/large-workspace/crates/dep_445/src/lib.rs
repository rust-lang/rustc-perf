pub fn code() {
    println!("Hello from dep_445");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_445");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_445: {t}");
}

pub fn foo() {
    dep_165::code();
    dep_165::code_inlined();
    dep_165::code_generic(1u32);
}
