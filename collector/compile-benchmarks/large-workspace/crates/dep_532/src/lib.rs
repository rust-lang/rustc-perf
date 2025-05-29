pub fn code() {
    println!("Hello from dep_532");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_532");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_532: {t}");
}

pub fn foo() {
    dep_188::code();
    dep_188::code_inlined();
    dep_188::code_generic(1u32);
}
